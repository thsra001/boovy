// bevy 13.2

use avian3d::prelude::*;
use bevy::{
    app::AppExit, core_pipeline::Skybox, math::vec3, pbr::CascadeShadowConfigBuilder, prelude::*,
    window::ExitCondition,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::*;
// local modules
mod player;
use player::LocalPlayerManager;
mod Bui;
use Bui::CreatorUi;
mod part;
use part::{part_factory, MaterialType, PartUtils, PropType, Scale};
mod Butils;
use std::f32::consts::PI;
use Butils::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Boovy creator".into(),
                ..default()
            }),
            exit_condition: ExitCondition::OnPrimaryClosed, // Close our app when all windows close
            close_when_requested: true,
        }))
        .enable_state_scoped_entities::<BoovyStates>()
        .add_plugins((
            WorldInspectorPlugin::new(),
            LocalPlayerManager,
            ThirdPersonCameraPlugin,
            PartUtils,
            CreatorUi,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            LoadButils,
        ))
        .add_systems(Update, kys)
        .add_systems(OnEnter(BoovyStates::Editor), test_setup)
        .run();
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum BoovyStates {
    //#[default]
    Loading, //preloading before showing app
    #[default]
    Menu, //  select game to edit
    Editor,  // editor to edit game
    Game,    // game testing
}

fn test_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut asset_server: Res<AssetServer>,
) {
    let scene = commands.spawn(Name::new("scene"));
    // square base
    let bonk = part_factory(
        PropType::BasicProp,
        &mut commands,
        &mut materials,
        &mut meshes,
    );
    commands.entity(bonk).insert((
        RigidBody::Static,
        Name::new("ground"),
        Position(vec3(0.0, -1.0, 0.0)),
        Scale(Vec3 {
            x: 200.0,
            y: 10.0,
            z: 100.0,
        }),
        MaterialType::Grass,
    ));
    //cube
    let cubis = part_factory(
        PropType::BasicProp,
        &mut commands,
        &mut materials,
        &mut meshes,
    );
    commands
        .entity(cubis)
        .insert((Name::new("cube"), Position(vec3(0.0, 1.0, 0.0))));

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

    // camera with ambientlight(env)
    let skypath = asset_server.load("cube.ktx2");
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(9.5, 6.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        ThirdPersonCamera {
            offset_enabled: true,
            offset: Offset::new(0.0, 0.8),
            zoom: Zoom::new(1.5, 100.0),
            cursor_lock_key: KeyCode::ControlLeft,
            ..default()
        },
        Skybox {
            image: skypath.clone(),
            brightness: 500.0,
        },
        EnvironmentMapLight {
            diffuse_map: asset_server.load("cube.ktx2"),
            specular_map: asset_server.load("cube.ktx2"),
            intensity: 100.0,
        },
        Name::new("camera"),
    ));
}
//fn serialis() {}
fn kys(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
