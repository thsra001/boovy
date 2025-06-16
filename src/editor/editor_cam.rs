//use crate::ui::routes::EditorSet;
use crate::player::LocalPlayer;
use crate::BoovyStates;
use avian3d::prelude::*;
use bevy::input::common_conditions::input_pressed;
use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use super::EditorSet;

pub struct EditorCameraManager;

#[derive(Reflect, Component)]
struct CamSpeed(f32);
#[derive(Component)]
struct ObjectType;
#[derive(Resource)]
pub struct Anim(Vec<Handle<AnimationClip>>);
#[derive(Resource)]
pub struct CamSens(pub f32);

impl Default for CamSens {
    fn default() -> Self {
        CamSens(0.5)
    }
}

impl Plugin for EditorCameraManager {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_editor_camera_keyboard.in_set(EditorSet),
                update_editor_camera_mouse
                    //.in_set(EditorSet)
                    .before(update_editor_camera_keyboard),
                //editor_camera_cursor_grab, //.in_set(EditorSet),
            ),
        ) // , player_anim add when fixed
        .register_type::<CamSpeed>()
        .init_resource::<CamSens>();
    }
}

// fn editor_camera_cursor_grab(
//     mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
//     mouse_button_state: Res<ButtonInput<MouseButton>>,
//     mut hold_cursor_pos: Local<Option<Vec2>>,
// ) {
//     let mut primary_window = q_windows.single_mut();

//     if mouse_button_state.just_released(MouseButton::Middle) {
//         primary_window.cursor_options.grab_mode = CursorGrabMode::None
//     }

//     if mouse_button_state.just_pressed(MouseButton::Middle) {
//         //*hold_cursor_pos = primary_window.cursor_position();
//         primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
//         //primary_window.cursor_options.visible = true;
//     }
// }
fn update_editor_camera_mouse(
    mut evr_motion: EventReader<MouseMotion>,
    mut cam_trans_query: Query<&mut Transform, With<Camera3d>>,
    mut pitch: Local<f32>,
    mut yaw: Local<f32>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    sens:Res<CamSens>,
    q_loca: Query<&LocalPlayer>,
) {
    let mut primary_window = q_windows.single_mut();
    let loca_valid_zoom: bool = if let Ok(loc) = q_loca.get_single() {
        if loc.zoom.current == 0.0 {
            true
        } else {
            false
        }
    } else {
        false
    };
    if mouse_buttons.pressed(MouseButton::Middle) || loca_valid_zoom {
        primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
        for ev in evr_motion.read() {
            //println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
            if let Ok(mut cam_trans) = cam_trans_query.get_single_mut() {
                *pitch += ev.delta.x * -0.005;
                *yaw += ev.delta.y * -0.005;
                *yaw = yaw.clamp(-80_f32.to_radians(), 80_f32.to_radians());
                cam_trans.rotation = Quat::from_euler(EulerRot::YXZ, *pitch * sens.0, *yaw * sens.0, 0.0)
            };
            primary_window.cursor_options.visible = false;
        }
    } else {
        //if mouse_buttons.just_released(MouseButton::Middle) {
        primary_window.cursor_options.grab_mode = CursorGrabMode::None;
        primary_window.cursor_options.visible = true;
    }
}
fn update_editor_camera_keyboard(
    input: Res<ButtonInput<KeyCode>>,
    mut get_cam: Query<&mut Transform, With<Camera3d>>,
) {
    let mut cam = match get_cam.get_single_mut() {
        Ok(c) => c,
        Err(_) => return,
    };
    let mut speed = 0.2;
    let mut dir = Vec3::ZERO;

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

    if input.pressed(KeyCode::KeyE) {
        dir += Vec3::Y
    }
    if input.pressed(KeyCode::KeyQ) {
        dir += Vec3::NEG_Y
    }

    if input.pressed(KeyCode::ShiftLeft) {
        speed = 0.6
    }
    if input.pressed(KeyCode::ControlLeft) {
        speed = 0.06
    }

    if dir.length_squared() > 0.0 {
        dir = dir.normalize();
        cam.translation += dir * speed;
        //info!("boi move: {dir}");
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
