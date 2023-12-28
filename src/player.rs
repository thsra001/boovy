use bevy::prelude::*;

pub struct LocalPlayerManager;

impl Plugin for LocalPlayerManager {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, make_player);
        app.add_systems(Update, player_movement);
    }
}

fn make_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    #[derive(Component)]
    struct LocaPlayer;
    #[derive(Component)]
    struct Speed(f32);
    #[derive(Component)]
    struct TypeSymbol;

    // character
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("char.glb#Scene0"),
            transform: Transform::from_xyz(-2.0, 2.25, 0.0),
            ..default()
        })
        .insert(Name::new("LocalPlayer"))
        .insert(LocaPlayer)
        .insert(Speed(10.0));
}


fn player_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    // mut get_player: Query<&mut Transform, With<LocaPlayer>>,
    // get_cam: Query<&Transform, (With<Camera3d>, Without<LocaPlayer>)>,
) {
    println!("noob");
} 
