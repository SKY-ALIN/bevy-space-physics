use bevy::prelude::*;

use super::ship_plugin::{SpaceShip, SpaceShipCameraTarget};
use super::space::SpaceObject;

#[derive(Component)]
pub struct GForceText;

#[derive(Component)]
pub struct CameraModeText;

pub fn setup_text(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 50.0,
                color: Color::WHITE,
                ..default()
            },
        )
        // .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        GForceText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "NA",
            TextStyle {
                font_size: 50.0,
                color: Color::WHITE,
                ..default()
            },
        )
        // .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        CameraModeText,
    ));
}

pub fn update_text(
    mut text_query: Query<&mut Text, With<GForceText>>,
    ship_query: Query<(&SpaceObject, &SpaceShip), With<SpaceShip>>,
) {
    let Ok(mut text) = text_query.get_single_mut() else { return };
    let Ok((object, ship)) = ship_query.get_single() else { return };

    const G: f32 = 9.81;

    let velocity = object.velocity.length();
    let angular_velocity = object.angular_velocity.length().to_degrees();

    let linear_overload = object.acceleration.length();
    let angular_overload = object.angular_velocity.length().powi(2) * ship.distance_from_center_to_pilot / G;
    let angular_acceleration_vector = Vec3::new(
        -object.angular_velocity.x * ship.distance_from_center_to_pilot,
        -object.angular_velocity.y * ship.distance_from_center_to_pilot,
        -object.angular_velocity.z * ship.distance_from_center_to_pilot,
    ).normalize() * angular_overload;
    let overload = (linear_overload + angular_acceleration_vector).length() / G;

    text.sections[0].value = format!("{overload:.2} G | {velocity:.2} m/s | {angular_velocity:.2} deg/s");
}

pub fn update_camera_mode_text(
    mut text_query: Query<&mut Text, With<CameraModeText>>,
    camera_query: Query<&SpaceShipCameraTarget>,
) {
    let Ok(mut text) = text_query.get_single_mut() else { return };
    let Ok(camera) = camera_query.get_single() else { return };

    let camera_mode = &camera.mode;

    text.sections[0].value = format!("Camera: {camera_mode:?}");
}
