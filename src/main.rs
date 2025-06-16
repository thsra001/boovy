// bevy 13.2
use avian3d::prelude::*;
use bevy::{
    app::AppExit,
    core_pipeline::Skybox,
    math::vec3,
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    state::commands,
    window::{ExitCondition, WindowResolution},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use bevy_third_person_camera::*;
// local modules
mod editor;
use bevy_skein::SkeinPlugin;
use editor::*;
mod player;
use player::LocalPlayerManager;
mod testplate;
use testplate::{testPlugin, Lake};
//mod ui;
//use ui::CreatorUi;

mod prop;
use prop::{BasicProp, MaterialType, MeshProp, PartUtils, Scale};
mod utils;
use std::f32::consts::PI;
use utils::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Boovy creator".into(),
                resolution: WindowResolution::default().with_scale_factor_override(0.9),
                ..default()
            }),
            exit_condition: ExitCondition::OnPrimaryClosed, // Close our app when all windows close
            close_when_requested: true,
        }))
        .init_state::<BoovyStates>()
        .enable_state_scoped_entities::<BoovyStates>()
        .register_type::<Lake>()
        .add_plugins((
            SkeinPlugin::default(),
            WorldInspectorPlugin::new(),
            PEditor,
            LocalPlayerManager,
            PartUtils,
            PhysicsPlugins::default().set(PhysicsInterpolationPlugin::extrapolate_all()),
            PhysicsDebugPlugin::default(),
            LoadButils,
            testPlugin,
        ))
        .add_systems(Update, kys)
        .add_systems(OnEnter(BoovyStates::EditorLoad), test_setup)
        .run();
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum BoovyStates {
    //#[default]
    Loading, //preloading before showing app
    Menu,    //  select game to edit
    #[default]
    EditorLoad, // laod scene first
    Editor,  // editor to edit game
    Game,    // game testing
}

fn test_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<BoovyStates>>,
    asset_server: Res<AssetServer>,
) {
    // todo: model
    let scene = commands.spawn(Name::new("scene"));
    // square base
    let bonk = BasicProp::create(&mut commands, &mut materials, &mut meshes);
    commands.entity(bonk).insert((
        RigidBody::Static,
        Name::new("ground"),
        Position(vec3(0.0, -15.0, 0.0)),
        Scale(Vec3 {
            x: 200.0,
            y: 10.0,
            z: 100.0,
        }),
        MaterialType::Grass,
    ));
    // meshtest
    let ojanga = MeshProp::create(
        String::from("meshes/error.glb"),
        &mut commands,
        &mut materials,
        &mut meshes,
        &asset_server,
    );
    commands.entity(ojanga).insert((
        Name::new("cooler mesh"),
        Position(vec3(-3.0, 5.0, 2.0)),
        //MaterialType::ConcreteTiles,
    ));
    //cube
    let obama = BasicProp::create(&mut commands, &mut materials, &mut meshes);
    commands.entity(obama).insert((
        Name::new("cooler cube"),
        Position(vec3(-3.0, 5.0, 2.0)),
        MaterialType::ConcreteTiles,
    ));
    //cube2
    let cubis = BasicProp::create(&mut commands, &mut materials, &mut meshes);
    commands
        .entity(cubis)
        .insert((Name::new("cube"), Position(vec3(3.0, 5.0, 2.0))));

    // Dirlight
    commands
        .spawn((
            DirectionalLight {
                illuminance: 1000.0,
                shadows_enabled: true,
                ..default()
            },
            Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            },
            CascadeShadowConfigBuilder {
                first_cascade_far_bound: 4.0,
                maximum_distance: 200.0,
                num_cascades: 4,
                ..default()
            }
            .build(),
        ))
        .insert(Name::new("DirectionalLight"));

    // camera with ambientlight(env)
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(9.5, 6.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        Skybox {
            image: asset_server.load("skybox.ktx2"),
            brightness: 500.0,
            ..default()
        },
        EnvironmentMapLight {
            diffuse_map: asset_server.load("skybox.ktx2"),
            specular_map: asset_server.load("specSkybox.ktx2"),
            intensity: 100.0,
            ..default()
        },
        Name::new("cam3d"),
    ));

    // loaded scene, now we go to editor
    next_state.set(BoovyStates::Editor)
}
//fn serialis() {}
fn kys(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
