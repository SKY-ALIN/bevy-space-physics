//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

use bevy::prelude::*;
use bevy_third_person_camera::*;

mod bevy_space_physics;
use bevy_space_physics::ship_plugin::{SpaceShipPlugin, SpaceShip, SpaceShipCameraTarget};
use bevy_space_physics::space::{SpacePlugin, SpaceObject};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ThirdPersonCameraPlugin)
        // .add_plugins(SpaceShipPlugin)
        .add_plugins((SpacePlugin, SpaceShipPlugin))
        .add_systems(Startup, setup)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        ..default()
    });

    let ship = meshes.add(shape::Box::new(1.0, 1.0, 2.5).into());

//     commands.spawn((
//         PbrBundle {
//             mesh: ship,
//             material: debug_material.clone(),
//             transform: Transform::from_xyz(
//                 0.0,
//                 2.0,
//                 0.0,
//             )
//             .with_rotation(Quat::from_rotation_x(-PI / 4.)),
//             ..default()
//         },
//         Player,
//     ));

    commands.spawn((
        ThirdPersonCamera {
            zoom: Zoom::new(4.0, 8.0),
            cursor_lock_key: KeyCode::Escape,
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
        SpaceObject::new(100.0),
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
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });
}
