use bevy::prelude::*;
use bevy_third_person_camera::*;

pub struct LocalPlayerManager;

#[derive(Component)]
struct LocaPlayer;
#[derive(Component)]
struct Speed(f32);
#[derive(Component)]
struct TypeSymbol;

impl Plugin for LocalPlayerManager {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, make_player);
        app.add_systems(Update, player_movement);
    }
}

fn make_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // character
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("char.glb#Scene0"),
            transform: Transform::from_xyz(-2.0, 1.46, 0.0),
            ..default()
        })
        .insert(Name::new("LocalPlayer"))
        .insert(LocaPlayer)
        .insert(Speed(10.0))
        .insert(ThirdPersonCameraTarget);
}

fn player_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut get_player: Query<(&mut Transform, &Speed), With<LocaPlayer>>,
    get_cam: Query<&Transform, (With<Camera3d>, Without<LocaPlayer>)>,
) {
    for (mut plr_move , plr_wroom) in get_player.iter_mut() {
        let cam = match get_cam.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("no cam :( {}", e)).unwrap(),
        };

        let mut dir = Vec3::ZERO;

        if input.pressed(KeyCode::W) {
            dir += cam.forward();
        }   

        if input.pressed(KeyCode::A) {
            dir += cam.left();
        }

        if input.pressed(KeyCode::S) {
            dir += cam.back();
        }

        if input.pressed(KeyCode::D) {
            dir += cam.right();
        }
        
        dir.y=0.0;
        let moves = dir.normalize_or_zero() * plr_wroom.0 * time.delta_seconds();
        plr_move.translation += moves;

        if dir.length_squared() > 0.0 {
            plr_move.look_to(dir, Vec3::Y)
        }
    }
}
