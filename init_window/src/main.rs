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
        .run();
}
