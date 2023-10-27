use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_kira_audio::prelude::*;

mod bevy_space_physics;
use bevy_space_physics::ship_plugin::{SpaceShipPlugin, SpaceShip, SpaceShipCameraTarget};
use bevy_space_physics::space::{SpacePlugin, SpaceObject};
use bevy_space_physics::g_force::{setup_text, update_text};
use bevy_space_physics::sound::SpaceShipSoundPlugin;

mod setup_effect;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ThirdPersonCameraPlugin)
        // .add_plugins(AudioPlugin)
        .add_plugins((SpacePlugin, SpaceShipPlugin, SpaceShipSoundPlugin))
        .add_systems(Startup, (setup, setup_text))
        .add_systems(Update, update_text)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ship = meshes.add(shape::Box::new(1.0, 1.0, 2.5).into());

    commands.spawn((
        ThirdPersonCamera {
            zoom: Zoom::new(4.0, 12.0),
            cursor_lock_key: KeyCode::Escape,
            offset_enabled: true,
            offset: Offset::new(0.0, 1.0),
            ..default()
        },
        Camera3dBundle::default(),
        SpaceShipCameraTarget,
    ));

    commands.spawn((
        PbrBundle {
            mesh: ship,
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..default()
        },
        ThirdPersonCameraTarget,
        SpaceObject::new(1000.0),
        SpaceShip::default(),
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(150.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });
}
