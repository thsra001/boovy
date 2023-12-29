use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::*;
mod player;
use player::LocalPlayerManager;

fn hello_world() {
    println!("biggas");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Thsra001's bevy gaum!1!!".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((
            WorldInspectorPlugin::new(),
            LocalPlayerManager,
            ThirdPersonCameraPlugin,
        ))
        .add_systems(Startup, hello_world)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // square base
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(20.).into()),
            material: materials.add(Color::rgb_u8(123, 23, 21).into()),
            ..default()
        })
        .insert(Name::new("floorPlane"));

    // cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(137, 52, 101).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Name::new("cube"));

    // light
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("pointLight"));
    // camera
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(9.5, 6.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            ThirdPersonCamera{
                offset_enabled: true,
                offset: Offset::new(0.5, 0.3),
                zoom: Zoom::new(1.5, 6.0),
                offset_toggle_key: KeyCode::T,
                cursor_lock_key: KeyCode::ShiftLeft,
                ..default()
            },
        ))
        .insert(Name::new("camera"));
    // delete 
    commands.spawn(RigidBody::Fixed)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)))
        .insert(Collider::capsule_y(0.5,0.5));
}
