use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::*;
use bevy_xpbd_3d::prelude::*;
mod player;
use player::LocalPlayerManager;
mod part;
use part::{part_factory, MaterialType, ObjectType, PartUtils};

#[derive(Component)]
struct DaSky;

use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Thsra001's bevy gaum!1!!".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
        .add_plugins((
            WorldInspectorPlugin::new(),
            LocalPlayerManager,
            ThirdPersonCameraPlugin,
            PartUtils,
        ))
        .add_systems(PostUpdate, skybox_move.in_set(CamTrans::After))
        .add_systems(Startup, (skybox_setup, setup))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // square base
    let bonk = part_factory(
        ObjectType::BasicObject,
        &mut commands,
        &mut materials,
        &mut meshes,
    );
    commands
        .entity(bonk)
        .insert((RigidBody::Static, Name::new("base")));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(25.0, 1.0, 25.0)),
            material: materials.add(Color::rgb_u8(23, 123, 21)),

            ..default()
        },
        Name::new("floorPlaneMesh"),
        RigidBody::Static,
        Collider::cuboid(25.0, 1.0, 25.0),
        MaterialType::Grass,
    ));

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 2.0, 2.0))),
            material: materials.add(Color::hex("a05525").unwrap()),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        Name::new("cube"),
        RigidBody::Dynamic,
        Collider::cuboid(2.0, 2.0, 2.0),
    ));

    // Dirlight
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 1000.0,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            },
            // I STOLE THIS FROM THE EXAMPLE PAGE
            // The default cascade config is designed to handle large scenes.
            // As this example has a much smaller world, we can tighten the shadow
            // bounds for better visual quality.
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 4.0,
                maximum_distance: 200.0,
                num_cascades: 4,
                ..default()
            }
            .into(),
            ..default()
        })
        .insert(Name::new("DirectionalLight"));
    // ambientLight
    commands.insert_resource(AmbientLight {
        color: Color::hex("#adc3f7").unwrap(),
        brightness: 500.0,
    });
    // camera
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(9.5, 6.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            ThirdPersonCamera {
                offset_enabled: true,
                offset: Offset::new(0.0, 0.8),
                zoom: Zoom::new(1.5, 10.0),
                cursor_lock_key: KeyCode::ShiftLeft,
                ..default()
            },
        ))
        .insert(Name::new("camera"));
}

fn skybox_setup(server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn(SceneBundle {
            scene: server.load("sky.glb#Scene0"),
            ..default()
        })
        .insert(DaSky)
        .insert(Name::new("Sky"));
}

fn skybox_move(
    get_cam: Query<&Transform, With<Camera3d>>,
    mut get_sky: Query<&mut Transform, (With<DaSky>, Without<Camera3d>)>,
) {
    let mut skoy = get_sky.get_single_mut().unwrap();
    let cam_pos = get_cam.get_single().unwrap();
    skoy.translation = cam_pos.translation;
}

fn make_part(PartType: &str) {
    println!("yup i hoops dis works");
}
/*
fn set_window_icon(

    // we have to use `NonSend` here
    // ill never understand this looool
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/logo.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    //  do it for all windows

    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }

    TODO: this got fucked, will try again if brave


}
*/
