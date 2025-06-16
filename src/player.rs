use avian3d::prelude::*;
use bevy::{
    color::palettes::css::GREEN,
    ecs::world::reflect,
    input::mouse::{MouseScrollUnit, MouseWheel},
    math::{ops::tanh, VectorSpace},
    prelude::*,
    render::view::visibility,
    state::commands,
    utils::info,
};

#[derive(Component, Reflect)]
#[reflect(Component)]
struct HideOnZoom;
use crate::{BoovyStates, EditorSet};
#[derive(Reflect)]
pub(crate) struct Zoom {
    max: f32,
    min: f32,
    pub current: f32,
}
impl Default for Zoom {
    fn default() -> Self {
        Zoom {
            max: 10.0,
            min: 0.0,
            current: 5.0,
        }
    }
}
#[derive(Component, Reflect)]
pub(crate) struct LocalPlayer {
    offset: Vec3,
    pub zoom: Zoom,
    jump_height: f32,
    max_speed: f32,
}
#[derive(Component, Reflect)]
struct Spring {
    strength: f32,
    damping: f32,
    ride_height: f32,
}
pub struct LocalPlayerManager;
impl Plugin for LocalPlayerManager {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(BoovyStates::Game),
            spawn_local_player.in_set(EditorSet),
        )
        .add_systems(
            Update,
            (
                update_player_cam,
                update_local_player.after(update_player_cam),
            ),
        )
        .register_type::<Spring>()
        .register_type::<LocalPlayer>();
    }
}
fn spawn_local_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn player character
    commands.spawn((
        SceneRoot(asset_server.load("char2.glb#Scene0")),
        Transform::from_xyz(0.0, 2.25, 10.0).with_scale(Vec3::splat(0.5)),
        Name::new("LocalPlayer"),
        LocalPlayer {
            offset: Vec3 {
                x: 0.0,
                y: 0.2,
                z: 0.0,
            },
            zoom: Zoom::default(),
            jump_height: 4.0,
            max_speed: 5.0,
        },
        Visibility::Inherited,
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
        LinearDamping(1.0),
        Collider::capsule(0.5, 1.0),
        //ColliderDensity(1000.0),
        RayCaster::new(Vec3::ZERO, Dir3::NEG_Y)
            .with_max_hits(1)
            .with_max_distance(0.95),
        Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
        Spring {
            strength: 200.0,
            damping: 6.0,
            ride_height: 0.75,
        },
        StateScoped(BoovyStates::Game),
        ExternalForce::new(Vec3::ZERO).with_persistence(false),
    ));
    // animations
    //commands.insert_resource(Anim(vec![asset_server.load("char2.glb#Animation0")]));
}
#[derive(Default, PartialEq)]
enum JumpState {
    // performed with code to stop jumping
    Disallowed,
    // can jump, on ground
    #[default]
    Eligable,
    // is jumping, still on ground so turn off spring
    OnGroundJumping,
    // is jumping off floor, so if toucey floor now, jump no more
    LiftoffJumping,
}
fn update_player_cam(
    input: Res<ButtonInput<KeyCode>>,
    mut get_cam: Query<&mut Transform, With<Camera3d>>,
    mut get_char: Query<
        (
            &mut ExternalForce,
            &mut LinearVelocity,
            &AngularVelocity,
            &mut ExternalAngularImpulse,
            &Transform,
            &Spring,
            &mut LocalPlayer,
            &mut Visibility,
        ),
        Without<Camera3d>,
    >,
    get_ray: Query<(&RayCaster, &RayHits)>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut gizmos: Gizmos,
    mut get_other_vel: Query<(&LinearVelocity, &ComputedCenterOfMass), Without<LocalPlayer>>,
    mut commands: Commands,
    mut jump_state: Local<JumpState>,
    time: Res<Time>,
) {
    let mut dir = Vec3::default();
    let mut cam = match get_cam.get_single_mut() {
        Ok(c) => c,
        Err(_) => return,
    };
    let (mut force, mut vel,angvel,mut ext_angvel, trans, sprink, mut loca, mut visi) = match get_char.get_single_mut() {
        Ok(c) => c,
        Err(_) => return,
    };
    let mut dist = None;
    let mut hitvel = None;
    let mut hitent: Option<Entity> = None;
    let mut hitpoint = None;
    for (ray, hits) in &get_ray {
        for hit in hits.iter_sorted() {
            dist = Some(hit.distance);
            hitvel = Some(get_other_vel.get(hit.entity));
            hitpoint = Some(ray.origin + *ray.direction * hit.distance);
            hitent = Some(hit.entity);
        }
    }

    if input.pressed(KeyCode::KeyW) {
        dir += *cam.forward();
    }

    if input.pressed(KeyCode::KeyA) {
        dir += *cam.left();
    }

    if input.pressed(KeyCode::KeyS) {
        dir += *cam.back();
    }

    if input.pressed(KeyCode::KeyD) {
        dir += *cam.right();
    }
    dir.y = 0.0;
    dir = dir.try_normalize().unwrap_or(Vec3::default());
    let ride_height = sprink.ride_height;
    if dist.is_some() {
        // jump fail: avoid looking stupid if jump velocity is insufficiant
        if dist.unwrap() <= ride_height * 0.8 && *jump_state == JumpState::OnGroundJumping {
            *jump_state = JumpState::Eligable
        } else if dist.unwrap() <= ride_height * 1.2 && *jump_state == JumpState::LiftoffJumping {
            *jump_state = JumpState::Eligable;
        }
        // original
        // dir.y = dist.unwrap() - ride_height;
        // info!("dist: {} vel: {}", dist.unwrap(), dir.y)

        // copy from vid
        let springstrength = sprink.strength;
        let springdamp = sprink.damping;

        let mut ogga_vel = Vec3::ZERO;
        let mut ogga_mass = None;
        if let Ok((other_vell, other_center_mass)) = hitvel.unwrap() {
            ogga_vel = other_vell.0;
            ogga_mass = Some(other_center_mass);
        };

        let raydir = trans.down();
        let dirvel = raydir.dot(vel.0);
        //info!(dirvel);
        let othervel = raydir.dot(ogga_vel);
        //info!(othervel);

        let relvel = dirvel - othervel;
        //info!(relvel);
        let offsetx = dist.unwrap() - ride_height;
        //info!(x);
        let spring = (offsetx * springstrength) - (relvel * springdamp);
        //info!(spring);
        //vel.0.y += -1.0 * spring
        //force.apply_force(Vec3 { x:0.0, y: (-1.0 * spring), z:0.0 });
        //dir.y = -1.0 * spring;
        if input.just_pressed(KeyCode::Space) && *jump_state == JumpState::Eligable {
            //dir.y += 90.0;
            vel.0.y += loca.jump_height;
            *jump_state = JumpState::OnGroundJumping
        }
        if *jump_state == JumpState::Eligable {
            force.apply_force(raydir * spring);

            if hitent.is_some() && dist.unwrap() <= ride_height * 1.2 {
                //force at pos raydir* -spring , rayhit.pos
                let mut imp = ExternalImpulse::new(Vec3::ZERO);
                imp.apply_impulse_at_point(
                    raydir * -spring,
                    hitpoint.unwrap(),
                    ogga_mass.unwrap().0,
                );
                commands.entity(hitent.unwrap()).insert(imp);
            }
        }
    } else if *jump_state == JumpState::OnGroundJumping {
        *jump_state = JumpState::LiftoffJumping;
    }
    dir *= loca.max_speed;

    let goal_vel = dir - vel.with_y(0.0);

    force.apply_force(goal_vel.clamp_length_max(loca.max_speed));

    // bing chilling
    let mut offset_size = loca.zoom.current;
    for ev in evr_scroll.read() {
        let pix = match ev.unit {
            MouseScrollUnit::Line => ev.y * 20.0,
            MouseScrollUnit::Pixel => ev.y,
        };
        offset_size -= pix / 50.0;
        offset_size = offset_size.clamp(loca.zoom.min, loca.zoom.max)
    }
    gizmos.line(
        trans.translation,
        trans.translation.with_y(trans.translation.y - 0.95),
        GREEN,
    );
    cam.translation = trans.translation + loca.offset + (*cam.back() * offset_size);
    loca.zoom.current = offset_size;
    if offset_size < 1.0 {
        *visi = Visibility::Hidden;
    } else {
        *visi = Visibility::Inherited;
    }

    
   //ext_angvel.apply_impulse((a * (rotRadian * 5.0))-(**angvel * 2.0));
    

}
fn update_local_player() {}
