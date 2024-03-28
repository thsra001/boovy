use bevy::{math::vec3, prelude::*};
use bevy_xpbd_3d::{
    components::{AngularDamping, LinearDamping, RigidBody},
    plugins::collision::Collider,
};
use std::fs;
use std::str::FromStr;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;
pub struct PartUtils;

impl Plugin for PartUtils {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (get_mats, setyp));
        app.add_systems(Update, (material_reflect, scale_reflect));
        app.register_type::<MaterialType>();
        app.register_type::<MatColour>();
        app.register_type::<MatMetalRough>();
        app.register_type::<MatNormal>();
        app.register_type::<ObjectType>();
        app.register_type::<Scale>();
    }
}
#[derive(Component)]
struct MaterialTemplate;
#[derive(Component)]
struct MaterialManager;
// loops trough mats folder and gets mats
#[derive(Component, Reflect, EnumString, PartialEq)]
pub enum MaterialType {
    Grass,
    Sand,
    Wood,
    WoodPlank,
    Concrete,
    ConcreteTiles,
}
// xpbd has postition, rotation but no scale wtf
#[derive(Component, Reflect)]
pub struct Scale(Vec3);
// type of object. a piece is a mesh, a sound
#[derive(Component, Reflect, IntoStaticStr)]
pub enum ObjectType {
    BasicObject,
    MeshObject,
}
#[derive(Component, Reflect)]
pub enum BasicObjectShape {
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
    let mat_holder = commands
        .spawn((Name::new("materialManager"), MaterialManager))
        .id();
    let paths = fs::read_dir("./assets/mats").unwrap();

    for path in paths {
        let fold = path.unwrap().file_name().into_string().unwrap();
        let pat = String::from("mats/") + &fold;

        println!("yup yup: {}", &pat);
        let chill = commands
            .spawn(Name::new(fold.clone()))
            .insert(MatColour(asset_server.load(pat.clone() + "/color.png")))
            .insert(MatMetalRough(
                asset_server.load(pat.clone() + "/metalRough.png"),
            ))
            .insert(MatNormal(asset_server.load(pat.clone() + "/normal.png")))
            .insert(MaterialTemplate)
            .insert(MaterialType::from_str(&fold).unwrap())
            .id();
        commands.entity(mat_holder).add_child(chill);
    }
}
pub fn part_factory(
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
                        collider: Collider::cuboid(1.0, 1.0, 1.0),
                        rigidbody_type: RigidBody::Dynamic,
                    },
                    CommonBundle {
                        name: Name::new(Into::<&'static str>::into(&var_obj_type)),
                        object_type: var_obj_type,
                    },
                    PbrBundle {
                        mesh: meshes.add(
                            //Cuboid::new(2.0, 2.0, 2.0)
                            Mesh::from(Cuboid::new(1.0, 1.0, 1.0))
                                .with_generated_tangents()
                                .unwrap(),
                        ),
                        material: materials.add(StandardMaterial { ..default() }),
                        ..default()
                    },
                    MaterialType::WoodPlank,
                    Scale(vec3(2.0, 2.0, 2.0)),
                    LinearDamping(0.25),
                    AngularDamping(0.2),
                ))
                .id();
        }
        ObjectType::MeshObject => return commands.spawn_empty().id(), //TODO: mesh part and other types
    };
}

fn material_reflect(
    mut object_query: Query<
        (&MaterialType, &Handle<StandardMaterial>), // query for what material to change into | query for objects material to modify
        (Changed<MaterialType>, Without<MaterialTemplate>),
    >, // the query for the object with changed material
    mut mat_query: Query<
        (&MaterialType, &MatColour, &MatNormal, &MatMetalRough),
        With<MaterialTemplate>,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>, // query for looping through material templates  TODO: find better way to get material template
) {
    for (mat_type, obj_mat) in &mut object_query.iter_mut() {
        for (typ, col, nor, metrou) in &mut mat_query.iter_mut() {
            if mat_type == typ {
                print!("yee");
                let temp = materials.get_mut(obj_mat).unwrap();
                temp.base_color_texture = Some(col.0.clone());
                temp.normal_map_texture = Some(nor.0.clone());
                temp.metallic_roughness_texture = Some(metrou.0.clone());
            }
        }
    }
}
fn scale_reflect(
    mut object_query: Query<
        (&Scale, &mut Transform), // query for what material to change into | query for objects material to modify
        Changed<Scale>,
    >,
) {
    for (siza, mut tran) in &mut object_query.iter_mut() {
        tran.scale = siza.0
    }
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
