use bevy::prelude::*;
use bevy::color::palettes::css::{DARK_GRAY, GREEN, PURPLE, RED, WHITE, YELLOW};
use bevy_math::Dir3;
use bevy_hanabi::prelude::*;
use bevy_editor_pls::prelude::*;

mod bevy_space_physics;
use bevy_space_physics::player::{CameraPlugin, SpaceShip, SpaceShipCameraTarget, SpaceShipPlugin,};
use bevy_space_physics::physics::{SpacePlugin, SpaceObject};
use bevy_space_physics::text::DataDysplayPlugin;
use bevy_space_physics::sound::SpaceShipSoundPlugin;


mod setup_effect;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // .add_plugins(EditorPlugin::default())
        // .add_plugins(AudioPlugin)
        .add_plugins(HanabiPlugin)
        .add_plugins((SpacePlugin, SpaceShipPlugin, SpaceShipSoundPlugin, CameraPlugin, DataDysplayPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, update_gismos)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    commands.spawn((Camera3dBundle::default(), SpaceShipCameraTarget::default()));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(0, 0, 255)),
            transform: Transform::from_xyz(15.0, 15.0, 15.0),
            ..default()
        },
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
            material: materials.add(Color::srgb_u8(0, 0, 255)),
            transform: Transform::from_xyz(-15.0, 15.0, 15.0),
            ..default()
        },
    ));

    commands.spawn(SpotLightBundle {
        spot_light: SpotLight {
            shadows_enabled: true,
            range: 300.0,
            intensity: 50000000.0,
            color: PURPLE.into(),
            // outer_angle: std::f32::consts::PI / 4.0,
            // inner_angle: std::f32::consts::PI / 4.0 * 0.8,
            ..default()
        },
        transform: Transform::from_xyz(13.0, 16.0, 13.0).looking_at(Vec3::NEG_X * 1.5, Vec3::Y),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::WHITE),
        ..default()
    });

    let (_, light_config) = config_store.config_mut::<LightGizmoConfigGroup>();
    light_config.draw_all = true;
    light_config.color = LightGizmoColor::MatchLightColor;

}


fn update_gismos(player_query: Query<(&Transform, &SpaceObject, &SpaceShip), With<SpaceShip>>, mut gizmos: Gizmos) {
    let Ok((ship_transform, object, ship)) = player_query.get_single() else { return };
    gizmos.arrow(ship_transform.translation, ship_transform.translation + ship_transform.forward() * 3.0, GREEN);
    gizmos.arrow(ship_transform.translation, ship_transform.translation + ship_transform.up() * 1.0, GREEN);
    if ship.desired_movement_vector.length() > 0.0 {
        gizmos.arrow(ship_transform.translation, ship_transform.translation + ship_transform.rotation * ship.desired_movement_vector * 2.5, WHITE);
    }
    if object.velocity.length() > 0.0 {
        gizmos.arrow(ship_transform.translation, ship_transform.translation + object.velocity.normalize() * 2.5, YELLOW);
    }
    if object.acceleration.length() > 0.0 {
        gizmos.arrow(ship_transform.translation, ship_transform.translation + object.acceleration.normalize() * 2.5, RED);
    }
    let angular_velocity_direction = Dir3::new(object.angular_velocity.normalize_or_zero());
    if angular_velocity_direction.is_ok() {
        gizmos.circle(ship_transform.translation, angular_velocity_direction.unwrap(), 2.0, GREEN);
    }
    let angular_acceleration_direction = Dir3::new(object.angular_acceleration.normalize_or_zero());
    if angular_acceleration_direction.is_ok() {
        gizmos.circle(ship_transform.translation, angular_acceleration_direction.unwrap(), 2.0, RED);
    }
    let desired_angular_direction = Dir3::new(ship.desired_rotation_vector.normalize_or_zero());
    if desired_angular_direction.is_ok() {
        gizmos.circle(ship_transform.translation, ship_transform.rotation * desired_angular_direction.unwrap(), 2.0, WHITE);
    }
    gizmos.grid(Vec3::ZERO, Quat::from_rotation_x(std::f32::consts::PI / 2.0), UVec2::splat(50), Vec2::new(10.0, 10.0), DARK_GRAY);
}
