use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Bevy Invaders".to_string(),
            width: 640.0,
            height: 480.0,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player.system())
        .add_system(bug_movement.system())
        .add_system(laser_movement.system())
        .add_system(bug_zapper.system())
        .run();
}

struct Player {
    delta_x: f32,
}

#[derive(Copy, Clone)]
enum BugMovement {
    Left,
    Right,
    Down { n: f32, next_left: bool },
}

struct Bug {
    movement: BugMovement,
}

struct Laser;

fn player(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut query: Query<(&mut Player, &mut Transform, &Handle<TextureAtlas>)>,
) {
    const ACCELERATION: f32 = 1.0;
    const MAX_VELOCITY: f32 = 16.0;

    for (mut player, mut trans, atlas_handle) in query.iter_mut() {
        let mut firing = false;

        if keyboard_input.pressed(KeyCode::Left) {
            player.delta_x -= ACCELERATION;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            player.delta_x += ACCELERATION;
        }
        if keyboard_input.just_pressed(KeyCode::Space) {
            firing = true;
        }

        // Apply movement deltas
        player.delta_x = player.delta_x.clamp(-MAX_VELOCITY, MAX_VELOCITY);
        trans.translation.x += player.delta_x;
        trans.translation.x = trans.translation.x.clamp(-320.0, 320.0);

        // Decelerate
        player.delta_x *= 0.75;

        if firing {
            commands
                .spawn()
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: atlas_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        trans.translation.x,
                        trans.translation.y + 24.0,
                        0.0,
                    )),
                    sprite: TextureAtlasSprite::new(2),
                    ..Default::default()
                })
                .insert(Laser {});
        }
    }
}

fn bug_movement(mut query: Query<(&mut Bug, &mut Transform)>) {
    for (mut bug, mut trans) in query.iter_mut() {
        let mut new_movement = bug.movement;
        match bug.movement {
            BugMovement::Left => {
                trans.translation.x -= 2.0;
                if trans.translation.x < -300.0 {
                    new_movement = BugMovement::Down {
                        n: 12.0,
                        next_left: false,
                    };
                }
            }
            BugMovement::Right => {
                trans.translation.x += 2.0;
                if trans.translation.x > 300.0 {
                    new_movement = BugMovement::Down {
                        n: 12.0,
                        next_left: true,
                    };
                }
            }
            BugMovement::Down { n, next_left } => {
                trans.translation.y -= 2.0;
                new_movement = BugMovement::Down {
                    n: n - 1.0,
                    next_left,
                };
                if n < 1.0 {
                    new_movement = if next_left {
                        BugMovement::Left
                    } else {
                        BugMovement::Right
                    };
                }
            }
        }
        bug.movement = new_movement;
    }
}

fn laser_movement(mut query: Query<(Entity, &Laser, &mut Transform)>, mut commands: Commands) {
    for (entity, _, mut trans) in query.iter_mut() {
        trans.translation += Vec3::new(0.0, 4.0, 0.0);

        if trans.translation.y > 240.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn bug_zapper(
    laser_query: Query<(Entity, &Laser, &Transform)>,
    collider_query: Query<(Entity, &Bug, &Transform)>,
    mut commands: Commands,
) {
    for (entity, _, trans) in laser_query.iter() {
        let laser_pos = Vec2::new(trans.translation.x, trans.translation.y);
        for (bug_entity, _, bug_transform) in collider_query.iter() {
            let bug_pos = Vec2::new(bug_transform.translation.x, bug_transform.translation.y);

            if bug_pos.distance(laser_pos) < 24.0 {
                commands.entity(bug_entity).despawn();
                commands.entity(entity).despawn();
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Setup the sprite sheet
    let texture_handle = asset_server.load("spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Spawn the player
    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -220.0, 0.0)),
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        })
        .insert(Player { delta_x: 0.0 });

    // Spawn rows of enemies
    for bug_row in 0..4 {
        let y = 200.0 - (bug_row as f32 * 30.0);
        for bug_col in 0..20 {
            let x = -300.0 + (bug_col as f32 * 30.0);
            commands
                .spawn()
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                    sprite: TextureAtlasSprite::new(1),
                    ..Default::default()
                })
                .insert(Bug {
                    movement: if bug_row % 2 == 0 {
                        BugMovement::Left
                    } else {
                        BugMovement::Right
                    },
                });
        }
    }
}
