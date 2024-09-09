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
    pub move_forward_key: KeyCode,
    pub move_back_key: KeyCode,
    pub move_up_key: KeyCode,
    pub move_down_key: KeyCode,
    pub move_left_key: KeyCode,
    pub move_right_key: KeyCode,
    pub pitch_up_key: KeyCode,
    pub pitch_down_key: KeyCode,
    pub yaw_left_key: KeyCode,
    pub yaw_right_key: KeyCode,
    pub roll_left_key: KeyCode,
    pub roll_right_key: KeyCode,
    pub toggle_camera_mode: KeyCode,
}

impl Default for ControlKeys {
    fn default() -> Self {
        ControlKeys {
            move_forward_key: KeyCode::Space,
            move_back_key: KeyCode::KeyX,
            move_up_key: KeyCode::ArrowUp,
            move_down_key: KeyCode::ArrowDown,
            move_left_key: KeyCode::ArrowLeft,
            move_right_key: KeyCode::ArrowRight,
            pitch_up_key: KeyCode::KeyS,
            pitch_down_key: KeyCode::KeyW,
            yaw_left_key: KeyCode::KeyA,
            yaw_right_key: KeyCode::KeyD,
            roll_left_key: KeyCode::KeyQ,
            roll_right_key: KeyCode::KeyE,
            toggle_camera_mode: KeyCode::KeyV,
        }
    }
}

#[derive(Component)]
pub struct SpaceShip {
    pub rotation_per_second: f32,
    pub main_thruster_power: f32,
    pub side_thrusters_power: f32,
    pub control_keys: ControlKeys,
    pub distance_from_center_to_pilot: f32,
}

impl Default for SpaceShip {
    fn default() -> Self {
        SpaceShip {
            rotation_per_second: 30.0,
            main_thruster_power: 10000.0,
            side_thrusters_power: 3000.0,
            control_keys: ControlKeys::default(),
            distance_from_center_to_pilot: 2.0,
        }
    }
}

#[derive(Debug)]
pub enum CameraMode {
    Absolute,
    Relative,
}

impl CameraMode {
    fn next(&self) -> Self {
        match self {
            CameraMode::Absolute => CameraMode::Relative,
            CameraMode::Relative => CameraMode::Absolute,
        }
    }
}

#[derive(Component)]
pub struct SpaceShipCameraTarget {
    pub mode: CameraMode,
}

impl Default for SpaceShipCameraTarget {
    fn default() -> Self {
        SpaceShipCameraTarget {
            mode: CameraMode::Absolute,
        }
    }
}

fn control_ship(
    mut ship_query: Query<(&SpaceShip, &Transform, &mut SpaceObject)>,
    mut camera_query: Query<&mut SpaceShipCameraTarget>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok((ship, ship_transform, mut object)) = ship_query.get_single_mut() else { return };
    let Ok(mut camera) = camera_query.get_single_mut() else { return };

    let mass = object.mass;

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

    let mut angular_acceleration = Vec3::ZERO;

    if keys.pressed(ship.control_keys.pitch_up_key) {
        angular_acceleration += ship_transform.right() * 1.0;
    }

    if keys.pressed(ship.control_keys.pitch_down_key) {
        angular_acceleration += ship_transform.left() * 1.0;
    }

    if keys.pressed(ship.control_keys.yaw_left_key) {
        angular_acceleration += ship_transform.up() * 1.0;
    }

    if keys.pressed(ship.control_keys.yaw_right_key) {
        angular_acceleration += ship_transform.down() * 1.0;
    }

    if keys.pressed(ship.control_keys.roll_left_key) {
        angular_acceleration += ship_transform.forward() * 1.0;
    }

    if keys.pressed(ship.control_keys.roll_right_key) {
        angular_acceleration += ship_transform.back() * 1.0;
    }

    object.acceleration = acceleration;
    object.angular_acceleration = angular_acceleration;

    if keys.just_pressed(ship.control_keys.toggle_camera_mode) {
        let new_mode = camera.mode.next();
        camera.mode = new_mode;
    }
}
