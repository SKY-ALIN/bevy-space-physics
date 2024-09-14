use bevy::prelude::*;


pub struct SpacePlugin;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        // app;
        app.add_systems(Update, law_of_conservation_of_self_momentum);
    }
}

#[derive(Component)]
pub struct SpaceObject {
    pub mass: f32,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub angular_velocity: Vec3,
    pub angular_acceleration: Vec3,
}

impl SpaceObject {
    pub fn new(mass: f32) -> Self {
        SpaceObject {
            mass,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            angular_acceleration: Vec3::ZERO,
        }
    }
}

fn law_of_conservation_of_self_momentum(
    mut ship_query: Query<(&mut SpaceObject, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut object, mut transform) in  &mut ship_query {

        let acceleration = object.acceleration;
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
