use avian3d::prelude::*;
use bevy::state::commands;
use bevy::{math::vec3, prelude::*};
use bevy_inspector_egui::InspectorOptions;
use std::fs;
use std::str::FromStr;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;
mod basicprop;
mod meshprop;
pub use basicprop::*;
pub use meshprop::*;
pub struct PartUtils;

impl Plugin for PartUtils {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (get_mats, load_error_mesh));
        app.add_systems(Update, (material_reflect, scale_reflect,manifest_mesh::<()>));
        app.register_type::<MaterialType>();
        app.register_type::<BasicPropShape>();
        app.register_type::<MatColour>();
        app.register_type::<MatMetalRough>();
        app.register_type::<MatNormal>();
        app.register_type::<PropType>();
        app.register_type::<Scale>();
    }
}
#[derive(Component)]
struct MaterialTemplate;
#[derive(Component)]
struct MaterialManager;
// loops trough mats folder and gets mats
#[derive(Component, Reflect, EnumString, PartialEq, Default)]
pub enum MaterialType {
    Grass,
    Sand,
    Wood,
    WoodPlank,
    #[default]
    Concrete,
    ConcreteTiles,
}
#[derive(Resource)]
pub struct ErrorMeshProp(Handle<Gltf>);
// xpbd has postition, rotation but no Scale wtf
#[derive(Component, Reflect, Default, InspectorOptions)]
pub struct Scale(pub Vec3);
// type of object. a piece is a mesh, a sound
#[derive(Component, Reflect, IntoStaticStr, Default)]
pub enum PropType {
    #[default]
    BasicProp,
    MeshProp,
}
#[derive(Component, Reflect, Default)]
pub enum BasicPropShape {
    #[default]
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
// manifesters
#[derive(Component)]
pub struct ManifestMesh3d<M:Asset>(M);
#[derive(Component)]
pub struct ManifestMeshMaterial3d<M:Asset>(M);
#[derive(Bundle, Default)]
pub struct CommonBundle {
    pub name: Name,
    pub prop_type: PropType,
}
#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    pub collider: Collider,
    pub rigidbody_type: RigidBody,
}
#[derive(Bundle)]
pub struct BasicPropBundle {
    pub common: CommonBundle,
    pub physics: PhysicsBundle,
    pub shape: BasicPropShape,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
    pub material_type: MaterialType,
    pub scale: Scale,
}
fn load_error_mesh(mut commands: Commands, ass: Res<AssetServer>) {
    let gltf = ass.load("meshes/error.glb");
    commands.insert_resource(ErrorMeshProp(gltf));
}
//  mcolour: asset_server.load(pat.clone() + "/color.ktx2")
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

fn shape_reflect(
    mut shape_query: Query<
        (&BasicPropShape, &Mesh3d), // query for what material to change into | query for Props material to modify
        Changed<BasicPropShape>,
    >,
    meshes: &mut ResMut<Assets<Mesh>>, // the query for the Prop with changed material
) {
    for (shape_type, mut mesh_ref) in &mut shape_query.iter_mut() {
        let mut temp = meshes.get_mut(mesh_ref).unwrap();

        let mut shape = match shape_type {
            BasicPropShape::Cube => Mesh::from(Cuboid::new(1.0, 1.0, 1.0)),
            BasicPropShape::Cyllinder => Mesh::from(Cylinder::new(1.0, 1.0)),
            BasicPropShape::Sphere => Mesh::from(Sphere::new(1.0)),
            BasicPropShape::Wedge => Mesh::from(Sphere::new(1.0)),
        };
        temp = &mut shape
        //mesh_ref = &meshes.add(shape.with_generated_tangents().unwrap());
    }
}
fn material_reflect(
    mut prop_query: Query<
        (&MaterialType, &MeshMaterial3d<StandardMaterial>), // query for what material to change into | query for Props material to modify
        (Changed<MaterialType>, Without<MaterialTemplate>),
    >, // the query for the Prop with changed material
    mut mat_query: Query<
        (&MaterialType, &MatColour, &MatNormal, &MatMetalRough),
        With<MaterialTemplate>,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>, // query for looping through material templates  TODO: find better way to get material template
) {
    for (mat_type, obj_mat) in &mut prop_query.iter_mut() {
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
fn manifest_mesh_material<M:Asset>(mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut mat_query: Query<
        (Entity,&ManifestMeshMaterial3d<M>,&MeshMaterial3d<StandardMaterial>),
        Changed<ManifestMeshMaterial3d<M>>,
    >){
    for (entity,manifest,material) in &mut mat_query.iter_mut() {
      //material.0=materials.add(manifest.0)
      // MeshMaterial3d(materials.add(StandardMaterial::default())),
      commands.entity(entity).insert(MeshMaterial3d(materials.add(StandardMaterial::default())));
    }
}
fn manifest_mesh<M:Asset>(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mes_query:Query<Entity,Changed<ManifestMesh3d<M>>>){
    for (entity) in &mut mes_query.iter_mut() {
    commands.entity(entity).insert(Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))));
    }
}