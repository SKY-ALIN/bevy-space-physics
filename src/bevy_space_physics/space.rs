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
    pub rotation: Quat,
}

impl SpaceObject {
    pub fn new(mass: f32) -> Self {
        SpaceObject {
            mass,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            rotation: Quat::IDENTITY,
        }
    }
}

fn law_of_conservation_of_self_momentum(
    mut ship_query: Query<(&mut SpaceObject, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut object, mut transform) in  &mut ship_query {
        // println!("translation={:?}", transform.translation);
        // println!("object.rotation={:?}", object.rotation);

        let acceleration = object.acceleration;
        object.velocity += acceleration * time.delta_seconds();
        transform.translation += object.velocity * time.delta_seconds();

        // println!("{:?}", transform.rotation);
        // println!("{:?}", transform.rotation * object.rotation * time.delta_seconds());
        // transform.rotate_local_x(0.1);
        // transform.rotation = object.rotation;
        // let r = transform.rotation;
        // transform.rotation = (object.rotation - transform.rotation) * time.delta_seconds();
        transform.rotation = object.rotation;
        // transform.rotate(object.rotation);
        // transform.rotate_x(angle)
        // transform.rotate_local(object.rotation);
        // transform.rotation = object.rotation * time.delta_seconds();
    }
}
