//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

use std::f32::consts::PI;

use bevy::prelude::*;

use super::space::{SpacePlugin, SpaceObject};

pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        // app;
        app.add_systems(Update, control_ship);
    }
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
pub struct SpaceShip {
    lock_rotation_key: KeyCode,
    rotation_per_second: f32,
}

impl Default for SpaceShip {
    fn default() -> Self {
        SpaceShip {
            lock_rotation_key: KeyCode::ShiftLeft,
            rotation_per_second: 30.0,
        }
    }
}

#[derive(Component)]
pub struct SpaceShipCameraTarget;

fn control_ship(
    camera_query: Query<&Transform, With<SpaceShipCameraTarget>>,
    mut ship_query: Query<(&SpaceShip, &Transform, &mut SpaceObject)>, //, Without<SpaceShipCameraTarget>>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    // println!(camera.transform);
    let Ok(camera_transform) = camera_query.get_single() else { return };
    let Ok((ship, ship_transform, mut object)) = ship_query.get_single_mut() else { return };

    // let rotation_matrix = Mat3::from_quat(camera_transform.rotation);

    let camera_vec3: Vec3 = camera_transform.rotation.xyz();
    let ship_vec3: Vec3 = ship_transform.rotation.xyz();
    let diff_vec3 = camera_vec3 - ship_vec3;

    // println!("Ship Quat: {}", camera_transform.rotation);
    // println!("Ship: {}", diff_vec3 * PI / 180.0);
    println!("Diff: {}", camera_transform.rotation - ship_transform.rotation);
    let rotation_diff = camera_transform.rotation - ship_transform.rotation;

    if !keys.pressed(ship.lock_rotation_key) {
        // if diff_vec3.x > 0.01 {
        //     object.rotation *= 
        //     ship_transform.rotate_x(time.delta_seconds() * ship.rotation_per_second * PI / 180.0);
        // } else if diff_vec3.x < -0.01 {
        //     ship_transform.rotate_x(time.delta_seconds() * ship.rotation_per_second * PI / 180.0 * -1.0);
        // }

        // if diff_vec3.y > 0.01 {
        //     ship_transform.rotate_y(time.delta_seconds() * ship.rotation_per_second * PI / 180.0);
        // } else if diff_vec3.y < -0.01 {
        //     ship_transform.rotate_y(time.delta_seconds() * ship.rotation_per_second * PI / 180.0 * -1.0);
        // }

        // if ship_vec3.z > 0.01 {
        //     ship_transform.rotate_z(time.delta_seconds() * ship.rotation_per_second * PI / 180.0);
        // } else if diff_vec3.z < -0.01 {
        //     ship_transform.rotate_z(time.delta_seconds() * ship.rotation_per_second * PI / 180.0 * -1.0);
        // }
    }
        
    // println!("ship_transform: {}", ship_transform.translation);
    if keys.pressed(KeyCode::W) {
        // println!("{}", Vec3::Z * ship_transform.rotation.xyz() * time.delta_seconds());
        object.velocity += ship_transform.forward() * time.delta_seconds();
        // object.velocity += ship_transform.rotation.xyz() * object.mass * time.delta_seconds();
    }
    // println!("rotation: {}", ship_transform.rotation);
    println!("velocity: {}", ship_transform.forward() * time.delta_seconds());
}
