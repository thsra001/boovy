use avian3d::prelude::*;
use bevy::prelude::*;

use super::{BasicPropShape, ManifestMesh3d, ManifestMeshMaterial3d, MaterialType, PropType, Scale};

#[derive(Component)]
#[require(Collider(|| Collider::cuboid(1.0,1.0,1.0)))]
#[require(RigidBody(|| RigidBody::Dynamic))]
#[require(Name(|| Name::new("BasicProp")))]
#[require(PropType(|| PropType::BasicProp))]
#[require(MeshMaterial3d<StandardMaterial>)]
#[require(Mesh3d)]
#[require(MaterialType(|| MaterialType::Concrete))]
#[require(BasicPropShape(|| BasicPropShape::Cube))]
#[require(Scale(|| Scale(Vec3::splat(2.0))))]

pub struct BasicProp;

impl BasicProp {
    pub fn create(
        commands: &mut Commands,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> bevy::prelude::Entity {
        return commands
            .spawn((
                // PhysicsBundle {
                //     collider: Collider::cuboid(1.0, 1.0, 1.0),
                //     rigidbody_type: RigidBody::Dynamic,
                // },
                // CommonBundle {
                //     name: Name::new("BasicProp"),
                //     prop_type: PropType::BasicProp,
                // },
                // MeshMaterial3d(materials.add(StandardMaterial::default())),
                // Mesh3d(
                //     meshes.add(
                //         Cuboid::new(1.0, 1.0, 1.0)
                //             .mesh()
                //             .build()
                //             .with_generated_tangents()
                //             .expect("oh fuck cube tangents noooo"),
                //     ),
                // ),
                // MaterialType::Concrete,
                // BasicPropShape::Cube,
                // Scale(vec3(2.0, 2.0, 2.0)),
                // LinearDamping(0.25),
                // AngularDamping(0.2),
                ManifestMeshMaterial3d(StandardMaterial::default()),
                ManifestMesh3d(Cuboid::new(2.0, 2.0, 2.0).mesh().build().with_generated_tangents().expect("oh fuck cube tangents nooo")),
                BasicProp,
        ))
            .id();
    }
}
