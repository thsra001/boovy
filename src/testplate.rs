use avian3d::{prelude::{
    AngularDamping, Collider, ColliderConstructor, ColliderConstructorHierarchy, CollidingEntities,
    ExternalImpulse, LinearDamping, LinearVelocity, Position, RigidBody, Sensor,
}, sync::ancestor_marker::{AncestorMarker, AncestorMarkerPlugin}};
use bevy::{
    math::{vec3, VectorSpace},
    prelude::*,
    state::{commands, reflect},
};

use crate::prop::{
    BasicProp, BasicPropShape, CommonBundle, MaterialType, PhysicsBundle, PropType, Scale,
};
pub struct testPlugin;
impl Plugin for testPlugin {
    fn build(&self, app: &mut App) {
        app
            // make set for editor ui
            .register_type::<Lake>()
            .register_type::<Pole>()
            .register_type::<Grass>()
            .register_type::<Dock>()
            .add_systems(Startup, spawn)
            .add_systems(Update, water_update);
    }
}
// gltf comps
#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(CollidingEntities(||{CollidingEntities::default()}))]
pub struct Lake;
#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(RigidBody(||{RigidBody::Static}))]
#[require(Collider(||{Collider::cuboid(0.1, 1.65, 0.1)}))]
pub struct Pole;
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Grass;
#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Collider(||{Collider::compound(vec![(vec3(1.5,0.0,0.0),Quat::default(),Collider::cuboid(3.0, 0.2, 1.0)),(vec3(3.5, 0.0, 0.75),Quat::default(),Collider::cuboid(1.0, 0.2, 2.5))])}))]
pub struct Dock;
fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // water
    // let water = BasicProp::create(&mut commands, &mut materials, &mut meshes);
    // commands.entity(water).insert((
    //     Name::new("water"),
    //     Position(vec3(40.0, 1.0, 40.0)),
    //     Scale(Vec3 {
    //         x: 30.0,
    //         y: 2.0,
    //         z: 30.0,
    //     }),
    //     RigidBody::Static,
    //     CollidingEntities::default(),
    //     Sensor,
    // ));
    // still water
    //let mesh = Mesh::new(primitive_topology, asset_usage)
    commands.spawn((
        Position::from_xyz(0.0, 4.0, 0.0),
        // PhysicsBundle {
        //     collider: Collider::cuboid(30.0, 2.0, 30.0),
        //     rigidbody_type: RigidBody::Static,
        // },
        //CollidingEntities::default(),
        //Sensor,
        CommonBundle {
            name: Name::new("LakeScene"),
            prop_type: PropType::BasicProp,
        },
        SceneRoot(asset_server.load("lake.glb#Scene0")),
        //ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        //MaterialType::Concrete,
        //BasicPropShape::Cube,
        Scale(vec3(1.0, 1.0, 1.0)),
        // LinearDamping(0.25),
        // AngularDamping(0.2),
        Lake,
    ));
    commands.spawn((
        SceneRoot(asset_server.load("boat.glb#Scene0")),
        RigidBody::Dynamic,
    ));
    // crate test
    let boxbox = BasicProp::create(&mut commands, &mut materials, &mut meshes);
    commands
        .entity(boxbox)
        .insert((Name::new("water"), Position(vec3(0.0, 0.0, 0.0))));
}

fn water_update(
    q_water_contacts: Query<&CollidingEntities>,
   // mut q_items: Query<(Entity, &mut LinearVelocity, Option<&RigidBody>,Option<&Parent>),With<AncestorMarker<Collider>>>,
   mut q_items: Query<(Entity, &mut LinearVelocity, &RigidBody)>,
    mut commands: Commands,
) {
    for coll in &q_water_contacts {
        for coll2 in coll.0.iter() {
            if let Ok((ent, mut linvel, rigdbdy)) = q_items.get_mut(*coll2) {
                if *rigdbdy == RigidBody::Dynamic {
                    //info!("bob: {}", coll2);
                    //linvel.0.y += 0.17;
                    commands
                        .entity(ent)
                        .insert(ExternalImpulse::new(Vec3::Y * 1.5));
                }
            }
        }
    }
}
