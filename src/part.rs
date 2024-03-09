use bevy::prelude::*;
use bevy_xpbd_3d::{
    components::{AngularDamping, LinearDamping, RigidBody},
    plugins::collision::Collider,
};
//use bevy_rapier3d::prelude::*;
use std::fs;
use strum_macros::IntoStaticStr;
pub struct PartUtils;

impl Plugin for PartUtils {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (get_mats, setyp));
        app.register_type::<MaterialType>();
        app.register_type::<MatColour>();
        app.register_type::<MatMetalRough>();
        app.register_type::<MatNormal>();
        app.register_type::<ObjectType>();
    }
}

// loops trough mats folder and gets mats
#[derive(Component, Reflect)]
enum MaterialType {
    Grass,
    Sand,
    Wood,
    WoodPlank,
}
// type of object. a piece is a mesh, a sound
#[derive(Component, Reflect, IntoStaticStr)]
enum ObjectType {
    BasicObject,
    MeshObject,
}
#[derive(Component, Reflect)]
enum BasicObjectShape {
    Cube,
    Cyllinder,
    Wedge,
    Sphere,
}
#[derive(Component, Reflect)]
struct MatNormal(Handle<Image>);
#[derive(Component, Reflect)]
struct MatColour(Handle<Image>);
#[derive(Component, Reflect)]
struct MatMetalRough(Handle<Image>);
#[derive(Bundle)]
struct CommonBundle {
    name: Name,
    object_type: ObjectType,
}
#[derive(Bundle)]
struct PhysicsBundle {
    collider: Collider,
    rigidbody_type: RigidBody,
}
//  mcolour: asset_server.load(pat.clone() + "/color.png")
fn get_mats(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("yup");
    let mat_holder = commands.spawn(Name::new("materialManager")).id();
    let paths = fs::read_dir("./assets/mats").unwrap();

    for path in paths {
        let fold = path.unwrap().file_name().into_string().unwrap();
        let pat = String::from("mats/") + &fold;

        println!("yup yup: {}", &pat);
        let chill = commands
            .spawn(Name::new(fold))
            .insert(MatColour(asset_server.load(pat.clone() + "/color.png")))
            .insert(MatMetalRough(
                asset_server.load(pat.clone() + "/metalRough.png"),
            ))
            .insert(MatNormal(asset_server.load(pat.clone() + "/normal.png")))
            .id();
        commands.entity(mat_holder).add_child(chill);
    }
}
fn part_factory(
    var_obj_type: ObjectType,
    mut commands: &mut Commands,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
) -> bevy::prelude::Entity {
    match var_obj_type {
        ObjectType::BasicObject => {
            return commands
                .spawn((
                    PhysicsBundle {
                        collider: Collider::cuboid(2.0, 2.0, 2.0),
                        rigidbody_type: RigidBody::Dynamic,
                    },
                    CommonBundle {
                        name: Name::new(Into::<&'static str>::into(&var_obj_type)),
                        object_type: var_obj_type,
                    },
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
                        material: materials.add(StandardMaterial { ..default() }),
                        ..default()
                    },
                    MaterialType::WoodPlank,
                    LinearDamping(0.25),
                    AngularDamping(0.2),
                ))
                .id()
        }
        ObjectType::MeshObject => return commands.spawn_empty().id(), //TODO: mesh part and other types
    };
}
fn setyp(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cubis = part_factory(
        ObjectType::BasicObject,
        &mut commands,
        &mut materials,
        &mut meshes,
    );
    commands
        .entity(cubis)
        .insert(Transform::from_xyz(0.0, 5.0, 0.0));
}
