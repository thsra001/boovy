use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;


pub struct LocalPlayerManager;

#[derive(Component)]
struct LocaPlayer;
#[derive(Component)]
struct LocalPlayerCollider;
#[derive(Component)]
struct Speed(f32);
#[derive(Component)]
struct ObjectType;

impl Plugin for LocalPlayerManager {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, make_player);
        app.add_systems(PreUpdate, player_movement);
    }
}

fn make_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // character model
    let plr = commands
        .spawn(SceneBundle {
            scene: asset_server.load("char.glb#Scene0"),
            //transform: Transform::from_xyz(0.0, 0.25, -0.1),
            ..default()
        })
        .insert(Name::new("LocalPlayerMesh"))
        .insert(LocaPlayer)
        .insert(Speed(10.0))
        .id();

    // character physics collider
    let physics = commands
        .spawn(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(TransformBundle::from(Transform::from_xyz(-4.0, 2.2, 0.0)))
        .insert(Collider::round_cylinder(1.0, 0.4, 0.2))
        .insert(Name::new("localPlayer"))
        .insert(LocalPlayerCollider)
        .insert(ThirdPersonCameraTarget)
        .insert(InheritedVisibility::default())
        .id();

    commands.entity(physics).add_child(plr);
}

fn player_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut get_player: Query<(&mut Transform, &Speed), With<LocaPlayer>>,
    mut get_collider: Query<&mut Transform, (With<LocalPlayerCollider>, Without<LocaPlayer>)>,
    get_cam: Query<
        &Transform,
        (
            With<Camera3d>,
            Without<LocaPlayer>,
            Without<LocalPlayerCollider>,
        ),
    >,
) {
    for (mut plr_move, plr_wroom) in get_player.iter_mut() {
        let cam = match get_cam.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("no cam :( {}", e)).unwrap(),
        };
        let mut col_trans = match get_collider.get_single_mut() {
            Ok(c) => c,
            Err(e) => Err(format!("no plr_collider :( {}", e)).unwrap(),
        };

        let mut dir = Vec3::ZERO;
        let mut new_transform = col_trans.clone();
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
        if input.pressed(KeyCode::Space){

        }

        dir.y = 0.0;

        if dir.length_squared() > 0.0 {
            let moves = dir.normalize_or_zero() * plr_wroom.0 * time.delta_seconds();
            new_transform.look_to(dir, Vec3::Y);

            plr_move.rotation = plr_move
                .rotation
                .slerp(new_transform.rotation, 6.0 * time.delta_seconds());
            col_trans.translation += moves;
        }
    }
}
