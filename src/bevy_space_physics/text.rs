use bevy::prelude::*;

use super::player::{SpaceShip, SpaceShipCameraTarget, SpaceShipSettings};
use super::physics::SpaceObject;

pub struct DataDysplayPlugin;

impl Plugin for DataDysplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_text)
            .add_systems(Update, (update_metrics_text, update_settings_text));
    }
}

#[derive(Component)]
pub struct MetricsText;

#[derive(Component)]
pub struct SettingsText;

pub fn setup_text(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        MetricsText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "NA",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        SettingsText,
    ));
}

pub fn update_metrics_text(
    mut text_query: Query<&mut Text, With<MetricsText>>,
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

    text.sections[0].value = format!("Overload: {overload:.2} G\nVelocity: {velocity:.2} m/s\nAngular velocity: {angular_velocity:.2} deg/s");
}

pub fn update_settings_text(
    mut text_query: Query<&mut Text, With<SettingsText>>,
    camera_query: Query<&SpaceShipCameraTarget>,
    settings_query: Query<&SpaceShipSettings>,
) {
    let Ok(mut text) = text_query.get_single_mut() else { return };
    let Ok(camera) = camera_query.get_single() else { return };
    let Ok(settings) = settings_query.get_single() else { return };

    let camera_mode = &camera.mode;
    let rotation_stabilization_mode = &settings.rotation_stabilization;
    let movement_stabilization_mode = &settings.movement_stabilization;

    text.sections[0].value = format!("Camera: {camera_mode:?} | Rotation stabilization: {rotation_stabilization_mode:?} | Movement Stabilization: {movement_stabilization_mode:?}");
}
