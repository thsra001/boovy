use avian3d::parry::shape;
use avian3d::prelude::*;
use bevy::{math::vec3, prelude::*};
use bevy_inspector_egui::InspectorOptions;
use std::fs;
use std::str::FromStr;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;
pub struct PartUtils;

impl Plugin for PartUtils {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, get_mats);
        app.add_systems(Update, (material_reflect, scale_reflect));
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
    pub pbr: PbrBundle,
    pub material_type: MaterialType,
    pub scale: Scale,
}
impl Default for BasicPropBundle {
    fn default() -> Self {
        Self {
            common: CommonBundle {
                name: Name::new("BasicProp"),
                prop_type: PropType::BasicProp,
            },
            physics: PhysicsBundle {
                collider: Collider::cuboid(2.0, 2.0, 2.0),
                rigidbody_type: RigidBody::Dynamic,
            },
            shape: BasicPropShape::Cube,
            pbr: PbrBundle::default(),
            material_type: MaterialType::Concrete,
            scale: Scale(Vec3::splat(1.0)),
        }
    }
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
pub fn part_factory(
    var_obj_type: PropType,
    mut commands: &mut Commands,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
) -> bevy::prelude::Entity {
    match var_obj_type {
        PropType::BasicProp => {
            return commands
                .spawn((
                    PhysicsBundle {
                        collider: Collider::cuboid(1.0, 1.0, 1.0),
                        rigidbody_type: RigidBody::Dynamic,
                    },
                    CommonBundle {
                        name: Name::new(Into::<&'static str>::into(&var_obj_type)),
                        prop_type: var_obj_type,
                    },
                    PbrBundle {
                        mesh: meshes.add(
                            Mesh::from(Cuboid::new(1.0, 1.0, 1.0))
                                .with_generated_tangents()
                                .unwrap(),
                        ),
                        material: materials.add(StandardMaterial { ..default() }),
                        ..default()
                    },
                    MaterialType::Concrete,
                    BasicPropShape::Cube,
                    Scale(vec3(2.0, 2.0, 2.0)),
                    LinearDamping(0.25),
                    AngularDamping(0.2),
                ))
                .id();
        }
        PropType::MeshProp => return commands.spawn_empty().id(), //TODO: mesh part and other types
    };
}

fn shape_reflect(
    mut shape_query: Query<
        (&BasicPropShape, &Handle<Mesh>), // query for what material to change into | query for Props material to modify
        (Changed<BasicPropShape>),
    >,
    mut meshes: &mut ResMut<Assets<Mesh>>, // the query for the Prop with changed material
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
        (&MaterialType, &Handle<StandardMaterial>), // query for what material to change into | query for Props material to modify
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
