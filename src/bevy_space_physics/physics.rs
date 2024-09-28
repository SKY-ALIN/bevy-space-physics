use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSet;

pub struct SpacePlugin;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (law_of_conservation_of_self_momentum, gravitational_force).in_set(PhysicsSet));
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
    gravity_points_query: Query<(&SpaceObject, &Transform), With<GravityPoint>>,
    mut no_gravity_objects_query: Query<(&mut SpaceObject, &Transform), Without<GravityPoint>>,
) {
    const G: f32 = 6.67430e-11;  // 6.67430×10^−11 N⋅m2⋅kg−2
    for (mut object, transform) in no_gravity_objects_query.iter_mut() {
        object.gravitational_force = Vec3::ZERO;
        for (gravity_point_object, gravity_point_transform) in gravity_points_query.iter() {
            let distance_vec = gravity_point_transform.translation - transform.translation;
            let distance = distance_vec.length();
            let force_magnitude = G * gravity_point_object.mass * object.mass / (distance * distance);
            let force_direction = distance_vec.normalize_or_zero();
            let force = force_magnitude * force_direction;
            object.gravitational_force += force;
        }
    }
}

fn law_of_conservation_of_self_momentum(
    mut ship_query: Query<(&mut SpaceObject, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut object, mut transform) in  &mut ship_query {

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
