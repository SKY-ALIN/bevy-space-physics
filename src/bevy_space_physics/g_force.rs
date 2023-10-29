use bevy::prelude::*;

use super::ship_plugin::SpaceShip;
use super::space::SpaceObject;

#[derive(Component)]
pub struct GForceText;

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
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        GForceText,
    ));
}

pub fn update_text(
    mut text_query: Query<&mut Text, With<GForceText>>,
    ship_query: Query<&SpaceObject, With<SpaceShip>>,
) {
    let Ok(mut text) = text_query.get_single_mut() else { return };
    let Ok(object) = ship_query.get_single() else { return };
    let acceleration = object.acceleration.length();
    let velocity = object.velocity.length();
    text.sections[0].value = format!("{acceleration:.2} G | {velocity:.2} m/s");
}
