use bevy::{prelude::*, transform::commands};
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
#[derive(Resource)]
pub struct Anim(Vec<Handle<AnimationClip>>);

impl Plugin for LocalPlayerManager {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, make_player);
        app.add_systems(PreUpdate, (player_movement, player_anim));
    }
}

fn make_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // character model
    let plr = commands
        .spawn(SceneBundle {
            scene: asset_server.load("char2.glb#Scene0"),
            //transform: Transform::from_xyz(0.0, 0.25, -0.1),
            ..default()
        })
        .insert(Name::new("LocalPlayerMesh"))
        .insert(LocaPlayer)
        .insert(Speed(130.0))
        .id();
    // animations
    commands.insert_resource(Anim(vec![asset_server.load("char2.glb#Animation0")]));
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
        .insert(Velocity {
            linvel: Vec3::new(1.0, 2.0, 3.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .id();

    commands.entity(physics).add_child(plr);
}

fn player_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut get_player: Query<(&mut Transform, &Speed), With<LocaPlayer>>,
    mut get_collider: Query<
        (&mut Transform, &mut Velocity),
        (With<LocalPlayerCollider>, Without<LocaPlayer>),
    >,
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
        let mut col_both = match get_collider.get_single_mut() {
            Ok(c) => c,
            Err(e) => Err(format!("no plr_collider :( {}", e)).unwrap(),
        };
        let (mut col_trans, mut col_vel) = col_both;

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

        dir.y = 0.0;
            if dir.length_squared() > 0.0 || input.pressed(KeyCode::Space) {
            let mut moves = dir.normalize_or_zero() * plr_wroom.0 * time.delta_seconds();
            new_transform.look_to(dir, Vec3::Y);

            plr_move.rotation = plr_move
                .rotation
                .slerp(new_transform.rotation, 6.0 * time.delta_seconds());
            if input.pressed(KeyCode::Space) {
                col_vel.linvel.y = 7.0;
            }
            col_vel.linvel.x = moves.x;
            col_vel.linvel.z = moves.z;
        }
        
    }
}
fn player_anim(
    animations: Res<Anim>,
    mut get_player_anim: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut daAnimator in &mut get_player_anim {
        daAnimator.play(animations.0[0].clone_weak()).repeat();
    }
}
