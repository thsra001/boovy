use avian3d::prelude::*;
use bevy::{math::vec3, prelude::*};

use super::{CommonBundle, ErrorMeshProp, MaterialType, PhysicsBundle, PropType, Scale};

#[derive(Component)]
pub struct MeshProp;

#[derive(Component)]
pub struct MeshUrl(String);
impl MeshProp {
    pub fn create(
        meshid: String,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        meshes: &mut ResMut<Assets<Mesh>>,
        asset_server: &Res<AssetServer>,
        //error_mesh: &Res<ErrorMeshProp>,
    ) -> bevy::prelude::Entity {
        return commands
            .spawn((
                SceneRoot(asset_server.load(meshid.clone() + "#Scene0")),
                // PhysicsBundle {
                //     collider: Collider::trimesh_from_mesh(
                //         asset_server.load(meshid.clone() + "#Primitive0"),meshes.get(id)
                //     )
                //     .expect("dang no collider"),
                //     rigidbody_type: RigidBody::Dynamic,
                // },
                CommonBundle {
                    name: Name::new("MeshProp"),
                    prop_type: PropType::MeshProp,
                },
                MeshMaterial3d(
                    materials.add(StandardMaterial::default()), //asset_server.load(meshid.clone() + "#Primitive0").material,
                ),
                Mesh3d(
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset("meshes/error.glb")),
                    //asset_server.load(meshid.clone() + "#Mesh0"),
                    // meshes.add(
                    //     Cuboid::new(1.0, 1.0, 1.0)
                    //         .mesh()
                    //         .build()
                    //         .with_generated_tangents()
                    //         .expect("oh fuck cube tangents noooo"),
                    // ),
                ),
                //MaterialType::Concrete,
                Scale(vec3(2.0, 2.0, 2.0)),
                MeshUrl(meshid),
                LinearDamping(0.25),
                AngularDamping(0.2),
            ))
            .id();
    }
}
