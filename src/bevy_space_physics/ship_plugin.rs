use bevy::prelude::*;
use bevy_math::{Quat, Vec3};

use super::space::SpaceObject;

pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        // app;
        app.add_systems(Update, control_ship);
    }
}

#[derive(Clone, Copy)]
pub struct ControlKeys {
    pub enable_mouse_rotation_key: KeyCode,
    pub move_forward_key: KeyCode,
    pub move_back_key: KeyCode,
    pub move_up_key: KeyCode,
    pub move_down_key: KeyCode,
    pub move_left_key: KeyCode,
    pub move_right_key: KeyCode,
}

impl Default for ControlKeys {
    fn default() -> Self {
        ControlKeys {
            enable_mouse_rotation_key: KeyCode::ControlLeft,
            move_forward_key: KeyCode::W,
            move_back_key: KeyCode::S,
            move_up_key: KeyCode::Space,
            move_down_key: KeyCode::X,
            move_left_key: KeyCode::A,
            move_right_key: KeyCode::D,
        }
    }
}

#[derive(Component)]
pub struct SpaceShip {
    // rotation_per_second: f32,
    pub main_thruster_power: f32,
    pub side_thrusters_power: f32,
    pub control_keys: ControlKeys,
}

impl Default for SpaceShip {
    fn default() -> Self {
        SpaceShip {
            // rotation_per_second: 30.0,
            main_thruster_power: 10000.0,
            side_thrusters_power: 3000.0,
            control_keys: ControlKeys::default(),
        }
    }
}

#[derive(Component)]
pub struct SpaceShipCameraTarget;

fn control_ship(
    camera_query: Query<&Transform, With<SpaceShipCameraTarget>>,
    mut ship_query: Query<(&SpaceShip, &Transform, &mut SpaceObject)>,
    // time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    let Ok(camera_transform) = camera_query.get_single() else { return };
    let Ok((ship, ship_transform, mut object)) = ship_query.get_single_mut() else { return };

    let mass = object.mass;
    // let rotation_matrix = Mat3::from_quat(camera_transform.rotation);

    // let camera_vec3: Vec3 = camera_transform.rotation.xyz();
    // let ship_vec3: Vec3 = ship_transform.rotation.xyz();
    // let diff_vec3 = camera_vec3 - ship_vec3;

    // println!("Ship Quat: {}", camera_transform.rotation);
    // println!("Ship: {}", diff_vec3 * PI / 180.0);f
    // println!("Diff: {}", camera_transform.rotation - ship_transform.rotation);
    // let rotation_diff = camera_transform.rotation - ship_transform.rotation;

    if keys.pressed(ship.control_keys.enable_mouse_rotation_key) {
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
        object.rotation = camera_transform.rotation;
        // object.rotation = camera_transform.rotation * Quat::inverse(ship_transform.rotation);
        // println!("object.rotation = {:?}", object.rotation);
    }

    let mut acceleration = Vec3::ZERO;

    if keys.pressed(ship.control_keys.move_forward_key) {
        acceleration += ship_transform.forward() * ship.main_thruster_power / mass;
    }
    if keys.pressed(ship.control_keys.move_back_key) {
        acceleration += ship_transform.back() * ship.side_thrusters_power / mass;
    }
    if keys.pressed(ship.control_keys.move_up_key) {
        acceleration += ship_transform.up() * ship.side_thrusters_power / mass;
    }
    if keys.pressed(ship.control_keys.move_down_key) {
        acceleration += ship_transform.down() * ship.side_thrusters_power / mass;
    }
    if keys.pressed(ship.control_keys.move_left_key) {
        acceleration += ship_transform.left() * ship.side_thrusters_power / mass;
    }
    if keys.pressed(ship.control_keys.move_right_key) {
        acceleration += ship_transform.right() * ship.side_thrusters_power / mass;
    }

    object.acceleration = acceleration;
}
