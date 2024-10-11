use std::marker::PhantomData;

use bevy::prelude::*;

use big_space::{
    precision::GridPrecision,
    reference_frame::local_origin::ReferenceFrames,
    world_query::GridTransform,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSet;

pub struct SpacePhysicsPlugin;

#[derive(Default)]
pub struct SpacePhysicsPluginBigSpace<P: GridPrecision>(PhantomData<P>);

impl Plugin for SpacePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                law_of_conservation_of_self_momentum,
                gravitational_force,
            ).in_set(PhysicsSet),
        );
    }
}

impl<P: GridPrecision> Plugin for SpacePhysicsPluginBigSpace<P> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                law_of_conservation_of_self_momentum_big_space::<P>,
                gravitational_force_big_space::<P>,
            ).in_set(PhysicsSet),
        );
    }
}

#[derive(Component)]
pub struct SpaceObject {
    pub mass: f32,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub angular_velocity: Vec3,
    pub angular_acceleration: Vec3,
    pub gravitational_force: Vec3,
}

impl SpaceObject {
    pub fn new(mass: f32) -> Self {
        SpaceObject {
            mass,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            angular_acceleration: Vec3::ZERO,
            gravitational_force: Vec3::ZERO,
        }
    }
}

#[derive(Component)]
pub struct GravityPoint;

fn gravitational_force(
    gravity_points_query: Query<(&SpaceObject, &GlobalTransform), With<GravityPoint>>,
    mut no_gravity_objects_query: Query<(&mut SpaceObject, &GlobalTransform), Without<GravityPoint>>,
) {
    const G: f32 = 6.67430e-11;  // 6.67430×10^−11 N⋅m2⋅kg−2
    for (mut object, transform) in no_gravity_objects_query.iter_mut() {
        object.gravitational_force = Vec3::ZERO;
        for (gravity_point_object, gravity_point_transform) in gravity_points_query.iter() {
            let distance_vec = gravity_point_transform.translation() - transform.translation();
            let distance = distance_vec.length();
            let force_magnitude = G * gravity_point_object.mass * object.mass / (distance * distance);
            let force_direction = distance_vec.normalize_or_zero();
            let force = force_magnitude * force_direction;
            object.gravitational_force += force;
        }
    }
}

fn gravitational_force_big_space<P: GridPrecision>(
    frames: ReferenceFrames<P>,
    gravity_points_query: Query<(&SpaceObject, Entity, GridTransform<P>), With<GravityPoint>>,
    mut no_gravity_objects_query: Query<(&mut SpaceObject, Entity, GridTransform<P>), Without<GravityPoint>>,
) {
    const G: f64 = 6.67430e-11;  // 6.67430×10^−11 N⋅m2⋅kg−2
    for (mut object, entity, grid_transform) in no_gravity_objects_query.iter_mut() {
        object.gravitational_force = Vec3::ZERO;
        for (gravity_point_object, gravity_point_entity, gravity_point_grid_transform) in gravity_points_query.iter() {
            let Some(gravity_point_reference_frame) = frames.parent_frame(gravity_point_entity) else {
                continue;
            };
            let Some(reference_frame) = frames.parent_frame(entity) else {
                continue;
            };
            let distance_vec = gravity_point_grid_transform.position_double(gravity_point_reference_frame) - grid_transform.position_double(reference_frame);
            let distance = distance_vec.length();
            let force_magnitude = G * (gravity_point_object.mass * object.mass) as f64 / (distance * distance);
            let force_direction = distance_vec.normalize_or_zero();
            let force = force_magnitude * force_direction;
            object.gravitational_force += force.as_vec3();
        }
    }
}

fn law_of_conservation_of_self_momentum(
    time: Res<Time>,
    mut object_query: Query<(&mut SpaceObject, &mut Transform)>,
) {
    for (mut object, mut transform) in  &mut object_query {

        let acceleration = object.acceleration + object.gravitational_force / object.mass;
        object.velocity += acceleration * time.delta_seconds();
        transform.translation += object.velocity * time.delta_seconds();

        let angular_acceleration = object.angular_acceleration;
        object.angular_velocity += angular_acceleration * time.delta_seconds();

        let delta_rotation = Quat::from_rotation_x(object.angular_velocity.x * time.delta_seconds())
            * Quat::from_rotation_y(object.angular_velocity.y * time.delta_seconds())
            * Quat::from_rotation_z(object.angular_velocity.z * time.delta_seconds());

        transform.rotation = delta_rotation * transform.rotation;
    }
}

fn law_of_conservation_of_self_momentum_big_space<P: GridPrecision>(
    time: Res<Time>,
    frames: ReferenceFrames<P>,
    mut object_query: Query<(&mut SpaceObject, Entity, GridTransform<P>)>,
) {
    for (mut object, entity, mut grid_transform) in  &mut object_query {
        let Some(reference_frame) = frames.parent_frame(entity) else {
            continue;
        };

        let acceleration = object.acceleration + object.gravitational_force / object.mass;
        object.velocity += acceleration * time.delta_seconds();
        let (delta_cell, delta_translation) = reference_frame.translation_to_grid(object.velocity * time.delta_seconds());
        *grid_transform.cell += delta_cell;
        grid_transform.transform.translation += delta_translation;

        let angular_acceleration = object.angular_acceleration;
        object.angular_velocity += angular_acceleration * time.delta_seconds();

        let delta_rotation = Quat::from_rotation_x(object.angular_velocity.x * time.delta_seconds())
            * Quat::from_rotation_y(object.angular_velocity.y * time.delta_seconds())
            * Quat::from_rotation_z(object.angular_velocity.z * time.delta_seconds());

        grid_transform.transform.rotation = delta_rotation * grid_transform.transform.rotation;
    }
}
