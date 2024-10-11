use bevy::{
    color::palettes::css::{GREEN, RED, WHITE, YELLOW}, core_pipeline::bloom::BloomSettings, math::DVec3, pbr::NotShadowCaster, prelude::*, render::camera::Exposure
};
use bevy_math::Dir3;
use big_space::{
    commands::BigSpaceCommands,
    reference_frame::ReferenceFrame,
    FloatingOrigin,
};
use bevy_hanabi::prelude::*;
use bevy_editor_pls::prelude::*;

mod bevy_space_physics;
use bevy_space_physics::player::{spawn_ship, AIPlayer, CameraPlugin, CameraSet, Player, SpaceShip, SpaceShipCameraTarget, SpaceShipPlugin, SpaceShipSettings};
use bevy_space_physics::physics::{SpacePhysicsPluginBigSpace, SpaceObject, GravityPoint};
use bevy_space_physics::text::DataDysplayPlugin;

mod setup_effect;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<TransformPlugin>().set(ImagePlugin::default_nearest()),
            big_space::BigSpacePlugin::<i64>::new(true),
            big_space::debug::FloatingOriginDebugPlugin::<i64>::default(),
        ))
        // .add_plugins(EditorPlugin::default())
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(HanabiPlugin)
        .add_systems(Startup, setup)
        .add_plugins((SpacePhysicsPluginBigSpace::<i64>::default(), SpaceShipPlugin, DataDysplayPlugin, CameraPlugin))
        .add_systems(Update, update_gizmos.after(CameraSet))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut effects: ResMut<Assets<EffectAsset>>,
    asset_server: Res<AssetServer>,
    mut config_store: ResMut<GizmoConfigStore>,
) {

    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
    //         material: materials.add(Color::srgb_u8(0, 0, 255)),
    //         transform: Transform::from_xyz(15.0, 15.0, 15.0),
    //         ..default()
    //     },
    //     SpaceObject::new(10.0),
    // ));

    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
    //         material: materials.add(Color::srgb_u8(0, 0, 255)),
    //         transform: Transform::from_xyz(-15.0, 15.0, 15.0),
    //         ..default()
    //     },
    //     SpaceObject::new(100.0),
    // ));

    commands.spawn_big_space(ReferenceFrame::<i64>::default(), |sun| {

        // Sun

        // root_frame.with_frame_default(|sun| {
            let sun_mass: f32 = 1.989e30;  // 1.989 × 10^30 kg
            sun.insert(Name::new("Sun"));
            sun.spawn_spatial((
                PbrBundle {
                    mesh: meshes.add(Sphere::new(696_340_000.0)),
                    material: materials.add(Color::srgb_u8(250, 160, 0)),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                },
                NotShadowCaster,
                SpaceObject::new(sun_mass),
                GravityPoint,
            ));

            // Earth

            let earth_orbit_radius: f64 = 149_597_871_000.0;
            let earth_radius: f32 = 6_371_000.0;
            let earth_mass: f32 = 5.972e24;  // 5.972 × 10^24 kg
            let geostationary_orbit_high: f64 = 35_786_000.0;

            let (earth_cell, earth_translation) = sun.frame().translation_to_grid(DVec3::new(0.0, 0.0, earth_orbit_radius));

            sun.with_frame_default(|earth| {
                earth.insert(Name::new("Earth"));
                earth.spawn_spatial((
                    PbrBundle {
                        mesh: meshes.add(Sphere::new(earth_radius)),
                        material: materials.add(Color::srgb_u8(0, 150, 255)),
                        transform: Transform::from_translation(earth_translation),
                        ..default()
                    },
                    SpaceObject::new(earth_mass),
                    GravityPoint,
                    earth_cell,
                ));
            });

            // Mars

            let mars_orbit_radius: f64 = 228_000_000_000.0;
            let mars_radius = 3_389_500.0;
            let mars_mass = 6.39e23; // 6.39 × 10^23 kg

            let (mars_cell, mars_translation) = sun.frame().translation_to_grid(DVec3::new(0.0, 0.0, mars_orbit_radius));

            sun.with_frame_default(|mars| {
                mars.insert(Name::new("Mars"));
                mars.spawn_spatial((
                    PbrBundle {
                        mesh: meshes.add(Sphere::new(mars_radius)),
                        material: materials.add(Color::srgb_u8(255, 150, 0)),
                        transform: Transform::from_translation(mars_translation),
                        ..default()
                    },
                    SpaceObject::new(mars_mass),
                    GravityPoint,
                    mars_cell,
                ));
            });


            let (player_cell, player_translation) = sun.frame().translation_to_grid(DVec3::new(geostationary_orbit_high, 0.0, earth_orbit_radius));

            sun.with_frame_default(|ship| {
                ship.insert((
                    Name::new("Player"),
                    player_cell,
                    Transform::from_translation(player_translation),
                    SpaceObject::new(1000.0),
                    SpaceShip::default(),
                    SpaceShipSettings::default(),
                    Player,
                ));

                ship.with_children(|children| {
                    children.spawn((
                        SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 1.0)),
                        SpatialListener::new(0.5),
                    ));
                });

                spawn_ship(
                    ship,
                    &mut meshes,
                    &mut materials,
                    &mut effects,
                    &asset_server,
                );
            });

            let (camera_cell, camera_translation) = sun.frame().translation_to_grid(DVec3::new(geostationary_orbit_high, 0.0, earth_orbit_radius + 10.0));

            sun.with_frame_default(|camera| {
                camera.insert((
                    Name::new("Camera"),
                    camera_cell,
                    Camera3dBundle {
                        transform: Transform::from_translation(camera_translation),
                        camera: Camera {
                            hdr: true,
                            ..default()
                        },
                        // exposure: Exposure::SUNLIGHT,
                        ..default()
                    },
                    SpaceShipCameraTarget::default(),
                    BloomSettings::default(),
                    FloatingOrigin,
                ));
            });

            let (ai_player_cell, ai_player_translation) = sun.frame().translation_to_grid(DVec3::new(geostationary_orbit_high, 0.0, earth_orbit_radius + 10000.0));

            sun.with_frame_default(|ship| {
                ship.insert((
                    Name::new("AI Player"),
                    ai_player_cell,
                    Transform::from_translation(ai_player_translation),
                    SpaceObject::new(1000.0),
                    SpaceShip::default(),
                    SpaceShipSettings::default(),
                    AIPlayer,
                ));

                spawn_ship(
                    ship,
                    &mut meshes,
                    &mut materials,
                    &mut effects,
                    &asset_server,
                );
            });
        // });
    });

    // commands.spawn(SpotLightBundle {
    //     spot_light: SpotLight {
    //         shadows_enabled: true,
    //         range: 300.0,
    //         intensity: 50000000.0,
    //         color: PURPLE.into(),
    //         // outer_angle: std::f32::consts::PI / 4.0,
    //         // inner_angle: std::f32::consts::PI / 4.0 * 0.8,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(13.0, 16.0, 13.0).looking_at(Vec3::NEG_X * 1.5, Vec3::Y),
    //     ..default()
    // });

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
    //     material: materials.add(Color::WHITE),
    //     ..default()
    // });

    // let (_, light_config) = config_store.config_mut::<LightGizmoConfigGroup>();
    // light_config.draw_all = true;
    // light_config.color = LightGizmoColor::MatchLightColor;

}


fn update_gizmos(player_query: Query<(&Transform, &SpaceObject, &SpaceShip), (With<SpaceShip>, With<Player>)>, mut gizmos: Gizmos) {
    for (ship_transform, object, ship) in player_query.iter() {
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
        // gizmos.grid(Vec3::ZERO, Quat::from_rotation_x(std::f32::consts::PI / 2.0), UVec2::splat(50), Vec2::new(10.0, 10.0), DARK_GRAY);
    }
}
