use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_xpbd_3d::prelude::*;

pub struct LocalPlayerManager;

#[derive(Component)]
struct LocaPlayer;
#[derive(Component)]
struct LocalPlayerCollider;
#[derive(Reflect, Component)]
struct Speed(f32);
#[derive(Component)]
struct ObjectType;
#[derive(Resource)]
pub struct Anim(Vec<Handle<AnimationClip>>);

impl Plugin for LocalPlayerManager {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, make_player);
        app.add_systems(Update, (player_movement)); // , player_anim add when fixed
        app.register_type::<Speed>();
    }
}

fn make_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // character model
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("char2.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 2.25, 10.0),
            ..default()
        },
        Name::new("LocalPlayer"),
        LocaPlayer,
        Speed(130.0),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        LinearDamping(0.75),
        Collider::capsule(1.5, 0.6),
        ThirdPersonCameraTarget,
    ));
    // animations
    commands.insert_resource(Anim(vec![asset_server.load("char2.glb#Animation0")]));
}
fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut get_player: Query<(&mut Transform, &Speed, &mut LinearVelocity), With<LocaPlayer>>,
    get_cam: Query<&Transform, (With<Camera3d>, Without<LocaPlayer>)>,
) {
    for (mut plr_move, plr_wroom, mut plr_vel) in get_player.iter_mut() {
        let cam = match get_cam.get_single() {
            Ok(c) => c,
            Err(e) => return,
        };

        let mut dir = Vec3::ZERO;
        let mut new_transform = plr_move.clone();
        if input.pressed(KeyCode::KeyW) {
            dir += *cam.forward();
        }

        if input.pressed(KeyCode::KeyA) {
            dir += *cam.left();
        }

        if input.pressed(KeyCode::KeyS) {
            dir += *cam.back();
        }

        if input.pressed(KeyCode::KeyD) {
            dir += *cam.right();
        }
        dir.y = 0.0;
        if dir.length_squared() > 0.0 {
            let mut moves = dir.normalize_or_zero() * plr_wroom.0 * time.delta_seconds();
            new_transform.look_to(dir, Vec3::Y);

            plr_move.rotation = plr_move
                .rotation
                .slerp(new_transform.rotation, 6.0 * time.delta_seconds());
            plr_vel.x = moves.x;
            plr_vel.z = moves.z;
        }
        if input.pressed(KeyCode::Space) {
            let mut moves = dir.normalize_or_zero() * plr_wroom.0 * time.delta_seconds();
            plr_vel.y = 7.0;
        }
    }
}
// fuck this man ill fix this later
// fn player_anim(
//     animations: Res<Anim>,
//     mut animations2: ResMut<Assets<AnimationClip>>,
//     mut get_player_anim: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
//     mut graphs: ResMut<Assets<AnimationGraph>>,
// ) {
//     for mut daAnimator in &mut get_player_anim {
//         let (graph, animation_index) =
//             AnimationGraph::from_clip(animations2.add(animations.0[0].clone_weak()));
//         daAnimator.play().repeat();
//     }
// }
// fn setup(
//     mut commands: Commands,
//     mut animations: ResMut<Assets<AnimationClip>>,
//     // You now need access to the `AnimationGraph` asset.
//     mut graphs: ResMut<Assets<AnimationGraph>>,
// ) {
//     let mut animation = AnimationClip::default();

//     // ...

//     // Create a new `AnimationGraph` and add the animation handle to it.
//     let (graph, animation_index) = AnimationGraph::from_clip(animations.add(animation));

//     let mut player = AnimationPlayer::default();
//     // Play the animation index, not the handle.
//     player.play(animation_index);

//     commands.spawn((
//         player,
//         // Add the new `AnimationGraph` to the assets, and spawn the entity with its handle.
//         graphs.add(graph),
//         // ...
//     ));
// }
