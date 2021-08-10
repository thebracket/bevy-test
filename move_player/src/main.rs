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
        .run();
}

struct Player {
    delta_x: f32,
}

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
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Setup the sprite sheet
    let texture_handle = asset_server.load("../../assets/spritesheet.png");
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
}
