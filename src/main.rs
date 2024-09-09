use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::MouseMotion;
use bevy_editor_pls::prelude::*;

mod bevy_space_physics;
use bevy_space_physics::ship_plugin::{SpaceShipPlugin, SpaceShip, SpaceShipCameraTarget};
use bevy_space_physics::space::{SpacePlugin, SpaceObject};
use bevy_space_physics::g_force::{setup_text, update_text, update_camera_mode_text};
use bevy_space_physics::sound::SpaceShipSoundPlugin;

mod setup_effect;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // .add_plugins(ThirdPersonCameraPlugin)
        // .add_plugins(EditorPlugin::default())
        // .add_plugins(AudioPlugin)
        .add_plugins((SpacePlugin, SpaceShipPlugin, SpaceShipSoundPlugin))
        .add_systems(Startup, (setup, setup_text))
        .add_systems(Update, (update_text, update_camera_mode_text, move_camera))
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ship = meshes.add(Cuboid::new(1.0, 1.0, 2.5));

    commands.spawn((
        PbrBundle {
            mesh: ship,
            material: materials.add(Color::srgb_u8(255, 0, 0)),
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..default()
        },
        SpaceObject::new(1000.0),
        SpaceShip::default(),
    ));

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

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::WHITE),
        ..default()
    });
}

fn move_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera: Query<&mut Transform, With<SpaceShipCameraTarget>>,
    player: Query<&Transform, (With<SpaceShip>, Without<SpaceShipCameraTarget>)>,

) {
    let Ok(player_transform) = player.get_single() else { return };
    let Ok(mut camera_transform) = camera.get_single_mut() else { return };
    let Ok(window) = window_query.get_single() else { return };

    let mut motion = Vec2::ZERO;
    for m in mouse_motion.read() {
        motion = m.delta;
    }

    let rotation = {
        if motion.length_squared() > 0.0 {
            let delta_x = -motion.x / window.width() * std::f32::consts::PI;
            let delta_y = -motion.y / window.height() * std::f32::consts::PI;
            let rotation = Quat::from_rotation_y(delta_x) * camera_transform.rotation;
            let rotation_with_pitch = rotation * Quat::from_rotation_x(delta_y);
            let up_vector = rotation_with_pitch * Vec3::Y;
            if up_vector.y > 0.0 {
                rotation_with_pitch
            } else {
                rotation
            }
        } else {
            camera_transform.rotation
        }
    };

    let rotation_metrix = Mat3::from_quat(rotation);
    //camera_transform.translation = player_transform.rotation * rotation_metrix.mul_vec3(Vec3::new(0.0,0.0, 10.0)) + player_transform.translation;
    camera_transform.translation = rotation_metrix.mul_vec3(Vec3::new(0.0,0.0, 10.0)) + player_transform.translation;
    // camera_transform.rotation = player_transform.rotation.inverse() * rotation;
    camera_transform.rotation = rotation;
}
