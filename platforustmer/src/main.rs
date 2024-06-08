use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_system(player_movement_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>,mut _materials: ResMut<Assets<ColorMaterial>>) {
    let texture_handle = asset_server.load("gopal.png");
    commands.spawn(SpriteBundle {
        texture: texture_handle,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            scale: Vec3::new(0.1, 0.1, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn player_movement_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform>,
) {
    let mut horizontal_movement = 0.0;
    if input.pressed(KeyCode::Left) {
        horizontal_movement -= 2.0;
    }
    if input.pressed(KeyCode::Right) {
        horizontal_movement += 2.0;
    }
    for mut transform in query.iter_mut() {
        transform.translation.x += horizontal_movement;
    }
}
