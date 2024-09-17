use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::MouseMotion;
use bevy_hanabi::prelude::*;

use super::physics::SpaceObject;

pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                control_ship,
                apply_thrusters,
                ship_rotation_full_stabilization,
                ship_rotation_player_aim_stabilization,
                ship_rotation_ai_aim_stabilization,
                ship_movement_stabilization,
            ));
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_camera);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AIPlayer;

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
    pub switch_camera_mode: KeyCode,
    pub switch_rotation_stabilization_mode: KeyCode,
    pub switch_movement_stabilization_mode: KeyCode,
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
            switch_camera_mode: KeyCode::KeyV,
            switch_rotation_stabilization_mode: KeyCode::ControlLeft,
            switch_movement_stabilization_mode: KeyCode::ShiftLeft,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RotationStabilization {
    No,
    Aiming,
    Full,
}

impl RotationStabilization {
    fn next(&self) -> Self {
        match self {
            RotationStabilization::No => RotationStabilization::Aiming,
            RotationStabilization::Aiming => RotationStabilization::Full,
            RotationStabilization::Full => RotationStabilization::No,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MovementStabilization {
    No,
    Full,
}

impl MovementStabilization {
    fn next(&self) -> Self {
        match self {
            MovementStabilization::No => MovementStabilization::Full,
            MovementStabilization::Full => MovementStabilization::No,
        }
    }
}

#[derive(Component)]
pub struct SpaceShipSettings {
    pub rotation_stabilization: RotationStabilization,
    pub movement_stabilization: MovementStabilization,
}

impl Default for SpaceShipSettings {
    fn default() -> Self {
        SpaceShipSettings {
            rotation_stabilization: RotationStabilization::No,
            movement_stabilization: MovementStabilization::No,
        }
    }
}

#[derive(Component)]
pub struct Thruster {
    pub force: f32,
    pub direction: Vec3,
}

#[derive(Component)]
pub struct SpaceShip {
    pub control_keys: ControlKeys,
    pub pilot_position: Vec3,
    pub desired_movement_vector: Vec3,
    pub desired_rotation_vector: Vec3,
}

impl Default for SpaceShip {
    fn default() -> Self {
        SpaceShip {
            control_keys: ControlKeys::default(),
            pilot_position: Vec3::new(0.0, 0.0, 1.0),
            desired_movement_vector: Vec3::ZERO,
            desired_rotation_vector: Vec3::ZERO,
        }
    }
}

#[derive(Debug)]
pub enum CameraMode {
    Absolute,
    Relative,
    Pov,
}

impl CameraMode {
    fn next(&self) -> Self {
        match self {
            CameraMode::Absolute => CameraMode::Relative,
            CameraMode::Relative => CameraMode::Pov,
            CameraMode::Pov => CameraMode::Absolute,
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
    mut ship_query: Query<&mut SpaceShip, With<Player>>,
    mut camera_query: Query<&mut SpaceShipCameraTarget>,
    mut settings_query: Query<&mut SpaceShipSettings, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut ship) = ship_query.get_single_mut() else { return };
    let Ok(mut camera) = camera_query.get_single_mut() else { return };
    let Ok(mut settings) = settings_query.get_single_mut() else { return };

    let mut desired_movement_vector = Vec3::ZERO;

    if keys.pressed(ship.control_keys.move_forward_key) {
        desired_movement_vector += Vec3::NEG_Z;
    }
    if keys.pressed(ship.control_keys.move_back_key) {
        desired_movement_vector += Vec3::Z;
    }
    if keys.pressed(ship.control_keys.move_up_key) {
        desired_movement_vector += Vec3::Y;
    }
    if keys.pressed(ship.control_keys.move_down_key) {
        desired_movement_vector += Vec3::NEG_Y;
    }
    if keys.pressed(ship.control_keys.move_left_key) {
        desired_movement_vector += Vec3::NEG_X;
    }
    if keys.pressed(ship.control_keys.move_right_key) {
        desired_movement_vector += Vec3::X;
    }

    let mut desired_rotation_vector = Vec3::ZERO;

    if keys.pressed(ship.control_keys.pitch_up_key) {
        desired_rotation_vector += Vec3::X;
    }

    if keys.pressed(ship.control_keys.pitch_down_key) {
        desired_rotation_vector += Vec3::NEG_X;
    }

    if keys.pressed(ship.control_keys.yaw_left_key) {
        desired_rotation_vector += Vec3::Y;
    }

    if keys.pressed(ship.control_keys.yaw_right_key) {
        desired_rotation_vector += Vec3::NEG_Y;
    }

    if keys.pressed(ship.control_keys.roll_left_key) {
        desired_rotation_vector += Vec3::Z;
    }
    
    if keys.pressed(ship.control_keys.roll_right_key) {
        desired_rotation_vector += Vec3::NEG_Z;
    }

    if settings.movement_stabilization == MovementStabilization::No {
        ship.desired_movement_vector = desired_movement_vector;
    }

    if settings.rotation_stabilization == RotationStabilization::No {
        ship.desired_rotation_vector = desired_rotation_vector;
    }

    if keys.just_pressed(ship.control_keys.switch_camera_mode) {
        let new_mode = camera.mode.next();
        camera.mode = new_mode;
    }

    if keys.just_pressed(ship.control_keys.switch_rotation_stabilization_mode) {
        let new_mode = settings.rotation_stabilization.next();
        settings.rotation_stabilization = new_mode;
    }

    if keys.just_pressed(ship.control_keys.switch_movement_stabilization_mode) {
        let new_mode = settings.movement_stabilization.next();
        settings.movement_stabilization = new_mode;
    }
}

fn apply_thrusters(
    mut ship_query: Query<(&SpaceShip, &Transform, &mut SpaceObject, &Children)>,
    thruster_query: Query<(&Thruster, &Transform, &Children)>,
    mut effect_query: Query<(&mut EffectSpawner, &mut EffectProperties)>,
) {
    for (ship, ship_transform, mut object, ship_children) in ship_query.iter_mut() {

        let mut movement_acceleration = Vec3::ZERO;
        let mut angular_acceleration = Vec3::ZERO;

        for (thruster, thruster_transform, thruster_children) in thruster_query.iter_many(ship_children) {
            let force_direction = (ship_transform.rotation * thruster.direction * -1.0).normalize();
            let r = thruster_transform.translation;
            let torque = r.cross(thruster.direction * -1.0 * thruster.force);

            let appliable_for_movement = if ship.desired_movement_vector.length() > 0.0 {
                thruster.direction.dot(ship.desired_movement_vector) < 0.0
            } else { false };

            let appliable_for_rotation = if ship.desired_rotation_vector.length() > 0.0 {
                torque.dot(ship.desired_rotation_vector) > 0.0
            } else { false };

            if appliable_for_movement || appliable_for_rotation {
                movement_acceleration += force_direction * thruster.force / object.mass;

                let (a, b, c) = (1.0f32, 1.0f32, 2.5f32);
                let moment_of_inertia = Vec3::new(
                    (b.powi(2) + c.powi(2)) * object.mass / 12.0,
                    (a.powi(2) + c.powi(2)) * object.mass / 12.0,
                    (a.powi(2) + b.powi(2)) * object.mass / 12.0,
                );
                // let inertia_tensor = Vec3::new(
                //     object.mass * (r.y.powi(2) + r.z.powi(2)),  // Ixx
                //     object.mass * (r.x.powi(2) + r.z.powi(2)),  // Iyy
                //     object.mass * (r.x.powi(2) + r.y.powi(2))   // Izz
                // );
                angular_acceleration += ship_transform.rotation * (torque / moment_of_inertia);
            }

            if appliable_for_movement || appliable_for_rotation {
                for &child in thruster_children {
                    if let Ok((mut effect_spawner, mut effect_properties)) = effect_query.get_mut(child) {
                        let Some(velocity_value) = effect_properties.get_stored("velocity_value") else { continue; };
                        effect_properties.set("velocity", (force_direction * -1.0 * velocity_value.as_scalar().as_f32() + object.velocity).into());
                        effect_spawner.set_active(true);
                    }
                }
            } else {
                for &child in thruster_children {
                    if let Ok((mut effect_spawner, _)) = effect_query.get_mut(child) {
                        effect_spawner.set_active(false);
                    }
                }
            }
        }
        object.acceleration = movement_acceleration;
        object.angular_acceleration = angular_acceleration;
    }
}

fn move_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera: Query<&mut Transform, With<SpaceShipCameraTarget>>,
    player: Query<&Transform, (With<Player>, Without<SpaceShipCameraTarget>)>,

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
    camera_transform.translation = rotation_metrix.mul_vec3(Vec3::new(0.0,1.5, 10.0)) + player_transform.translation;
    camera_transform.rotation = rotation;
}

fn ship_rotation_full_stabilization(
    mut ship_query: Query<(&SpaceObject, &Transform, &mut SpaceShip, &SpaceShipSettings)>,
) {
    const PERMISSIBLE_STABILIZATION_ERROR: f32 = 1.0;

    for (object, ship_transform, mut ship, settings) in ship_query.iter_mut() {

        if settings.rotation_stabilization != RotationStabilization::Full { continue; }


        if object.angular_velocity.length().to_degrees() < PERMISSIBLE_STABILIZATION_ERROR {
            ship.desired_rotation_vector = Vec3::ZERO;
            continue;
        }
        let stabilization_angular_vector = object.angular_velocity.normalize() * -1.0;

        // let stabilization_angular_vector_x = if object.angular_velocity.x.abs().to_degrees() > PERMISSIBLE_STABILIZATION_ERROR { -object.angular_velocity.x } else { 0.0 };
        // let stabilization_angular_vector_y = if object.angular_velocity.y.abs().to_degrees() > PERMISSIBLE_STABILIZATION_ERROR { -object.angular_velocity.y } else { 0.0 };
        // let stabilization_angular_vector_z = if object.angular_velocity.z.abs().to_degrees() > PERMISSIBLE_STABILIZATION_ERROR { -object.angular_velocity.z } else { 0.0 };
        // let stabilization_angular_vector = Vec3::new(stabilization_angular_vector_x, stabilization_angular_vector_y, stabilization_angular_vector_z).normalize();

        ship.desired_rotation_vector = ship_transform.rotation.inverse() * stabilization_angular_vector;
    }
}

fn ship_rotation_player_aim_stabilization(
    mut ship_query: Query<(&SpaceObject, &mut SpaceShip, &Transform, &SpaceShipSettings), With<Player>>,
    camera_query: Query<&Transform, With<SpaceShipCameraTarget>>,
) {
    let Ok((object, mut ship, player_transform, settings)) = ship_query.get_single_mut() else { return };
    let Ok(camera_transform) = camera_query.get_single() else { return };
    if settings.rotation_stabilization != RotationStabilization::Aiming { return }
    ship_rotation_aim_stabilization(object, &mut ship, player_transform, camera_transform.rotation);
}

fn ship_rotation_ai_aim_stabilization(
    mut ship_query: Query<(&SpaceObject, &mut SpaceShip, &Transform), With<AIPlayer>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    for (object, mut ship, ship_transform) in ship_query.iter_mut() {
        ship.desired_movement_vector = Vec3::NEG_Z;
        let target = Quat::from_rotation_arc(Vec3::Z, (ship_transform.translation - player_transform.translation).normalize());
        ship_rotation_aim_stabilization(object, &mut ship, ship_transform, target);
    }
}

fn ship_rotation_aim_stabilization(
    object: &SpaceObject,
    ship: &mut SpaceShip,
    player_transform: &Transform,
    target_rotation: Quat,
) {
    const MIN_ANGULAR_VELOCITY: f32 = std::f32::consts::PI / 12.0;  // 15 degrees / second
    const STABILIZATION_THRESHOLD: f32 = std::f32::consts::PI / 60.0;  // 3 degrees

    let desired_direction = (player_transform.rotation.inverse() * target_rotation).xyz().normalize();
    let current_velocity = object.angular_velocity.length();
    let max_angular_velocity = (player_transform.rotation.angle_between(target_rotation) / 2.0).max(MIN_ANGULAR_VELOCITY);  // from 15 to 120 degrees per sec

    if player_transform.rotation.angle_between(target_rotation) < STABILIZATION_THRESHOLD {
        if current_velocity > STABILIZATION_THRESHOLD {
            ship.desired_rotation_vector = player_transform.rotation.inverse() * object.angular_velocity.normalize_or_zero() * -1.0;
        } else {
            ship.desired_rotation_vector = Vec3::ZERO;
        }
    } else if current_velocity > max_angular_velocity {
        ship.desired_rotation_vector = player_transform.rotation.inverse() * object.angular_velocity.normalize_or_zero() * -1.0;
    } else {
        if object.angular_velocity.normalize_or_zero().dot(desired_direction) < 0.97 || current_velocity < MIN_ANGULAR_VELOCITY {
            let current_direction = object.angular_velocity.normalize_or_zero();
            let direction_difference = desired_direction - current_direction;
            ship.desired_rotation_vector = (current_direction + direction_difference).normalize();
        } else {
            ship.desired_rotation_vector = Vec3::ZERO;
        }
    }
}

fn ship_movement_stabilization(
    mut ship_query: Query<(&SpaceObject, &Transform, &mut SpaceShip, &SpaceShipSettings)>,
) {
    const PERMISSIBLE_STABILIZATION_ERROR: f32 = 0.3;  // 0.3 m/s
    for (object, ship_transform, mut ship, settings) in ship_query.iter_mut() {
        if settings.movement_stabilization == MovementStabilization::No { continue; }

        if object.velocity.length() < PERMISSIBLE_STABILIZATION_ERROR {
            ship.desired_movement_vector = Vec3::ZERO;
            continue;
        }

        let stabilization_vector = object.velocity.normalize() * -1.0;
        ship.desired_movement_vector = ship_transform.rotation.inverse() * stabilization_vector;
    }
}

fn create_side_thruster_effect() -> EffectAsset {
    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(0.1).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    writer.add_property("velocity_value", 5.0.into());
    let velocity = writer.add_property("velocity", Vec3::ZERO.into());
    let init_velocity = SetAttributeModifier::new(Attribute::VELOCITY, writer.prop(velocity).expr());

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::new(0.0, 0.0, 0.0)).expr(),
        radius: writer.lit(0.05).expr(),
        dimension: ShapeDimension::Volume,
    };

    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::splat(1.0));
    color_gradient.add_key(0.2, Vec4::splat(0.2));
    color_gradient.add_key(1.0, Vec4::splat(0.0));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec2::splat(0.01));
    size_gradient.add_key(1.0, Vec2::splat(0.1));

    EffectAsset::new(
        vec![1000],
        Spawner::rate(500.0.into()),
        writer.finish(),
    )
    .with_name("SideThrusterFlame")
    .init(init_pos)
    .init(init_age)
    .init(init_lifetime)
    .init(init_velocity)
    .render(ColorOverLifetimeModifier {
        gradient: color_gradient,
    })
    .render(
        SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        },
    )
    .render(OrientModifier {
        mode: OrientMode::FaceCameraPosition,
        rotation: Default::default(),
    })
}

fn create_main_thruster_effect() -> EffectAsset {
    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(0.2).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    writer.add_property("velocity_value", 15.0.into());
    let velocity = writer.add_property("velocity", Vec3::ZERO.into());
    let init_velocity = SetAttributeModifier::new(Attribute::VELOCITY, writer.prop(velocity).expr());

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::new(0.0, 0.0, 0.0)).expr(),
        radius: writer.lit(0.15).expr(),
        dimension: ShapeDimension::Volume,
    };

    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::splat(1.0));
    color_gradient.add_key(0.4, Vec4::new(1.0, 1.0, 0.0, 1.0));
    color_gradient.add_key(0.8, Vec4::new(1.0, 0.0, 0.0, 1.0));
    color_gradient.add_key(1.0, Vec4::splat(0.0));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec2::splat(0.05));
    size_gradient.add_key(0.3, Vec2::splat(0.1));
    size_gradient.add_key(1.0, Vec2::splat(0.3));

    EffectAsset::new(
        vec![1000],
        Spawner::rate(500.0.into()),
        writer.finish(),
    )
    .with_name("ThrusterFlame")
    .init(init_pos)
    .init(init_age)
    .init(init_lifetime)
    .init(init_velocity)
    .render(ColorOverLifetimeModifier {
        gradient: color_gradient,
    })
    .render(
        SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        },
    )
    .render(OrientModifier {
        mode: OrientMode::FaceCameraPosition,
        rotation: Default::default(),
    })
}

fn spawn_ship<T: Bundle>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    effects: &mut ResMut<Assets<EffectAsset>>,
    transform: Transform,
    boundle: T,
) {
    let ship_mesh = meshes.add(Cuboid::new(1.0, 1.0, 2.5));

    let mut ship = commands.spawn((
        PbrBundle {
            mesh: ship_mesh,
            material: materials.add(Color::srgb_u8(255, 0, 0)),
            transform: transform,
            ..default()
        },
        SpaceObject::new(1000.0),
        SpaceShip::default(),
        SpaceShipSettings::default(),
        boundle,
    ));
    
    ship.with_children(|parent| {

        // main thruster

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(0.0, 0.0, 1.5),
                ..default()
            },
            Thruster { force: 1000.0, direction: Vec3::Z },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_main_thruster_effect())),
                    transform: Transform::from_xyz(0.0, 0.0, 0.25),
                    ..default()
                }
            );
        });

        // side thrusters (top)

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(0.0, 0.55, -1.1),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::Y },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(0.0, 0.05, 0.0),
                    ..default()
                }
            );
        });

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(0.0, 0.55, 1.1),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::Y },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(0.0, 0.05, 0.0),
                    ..default()
                }
            );
        });

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(-0.4, 0.55, 0.0),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::Y },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(0.0, 0.05, 0.0),
                    ..default()
                }
            );
        });

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(0.4, 0.55, 0.0),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::Y },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(0.0, 0.05, 0.0),
                    ..default()
                }
            );
        });

        // side thrusters (bottom)

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(0.0, -0.55, -1.1),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::NEG_Y },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(0.0, -0.05, 0.0),
                    ..default()
                }
            );
        });

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(0.0, -0.55, 1.1),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::NEG_Y },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(0.0, -0.05, 0.0),
                    ..default()
                }
            );
        });

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(-0.4, -0.55, 0.0),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::NEG_Y },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(0.0, -0.05, 0.0),
                    ..default()
                }
            );
        });

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(0.4, -0.55, 0.0),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::NEG_Y },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(0.0, -0.05, 0.0),
                    ..default()
                }
            );
        });

        // side thrusters (left)

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(-0.55, 0.0, -1.1),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::NEG_X },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(-0.05, 0.0, 0.0),
                    ..default()
                }
            );
        });

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(-0.55, 0.0, 1.1),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::NEG_X },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(-0.05, 0.0, 0.0),
                    ..default()
                }
            );
        });

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(-0.55, 0.4, 0.0),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::NEG_X },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(-0.05, 0.0, 0.0),
                    ..default()
                }
            );
        });

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(-0.55, -0.4, 0.0),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::NEG_X },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(-0.05, 0.0, 0.0),
                    ..default()
                }
            );
        });

        // side thrusters (right)

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(0.55, 0.0, -1.1),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::X },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(0.05, 0.0, 0.0),
                    ..default()
                }
            );
        });

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(0.55, 0.0, 1.1),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::X },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(0.05, 0.0, 0.0),
                    ..default()
                }
            );
        });

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(0.55, 0.4, 0.0),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::X },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(0.05, 0.0, 0.0),
                    ..default()
                }
            );
        });

        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.1, 0.1, 0.1)),
                material: materials.add(Color::srgb_u8(50, 50, 50)),
                transform: Transform::from_xyz(0.55, -0.4, 0.0),
                ..default()
            },
            Thruster { force: 100.0, direction: Vec3::X },
        )).with_children(|thruster| {
            thruster.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(create_side_thruster_effect())),
                    transform: Transform::from_xyz(0.05, 0.0, 0.0),
                    ..default()
                }
            );
        });
    });
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    spawn_ship(&mut commands, &mut meshes, &mut materials, &mut effects, Transform::from_xyz(0.0, 10.0, 0.0), Player);
    spawn_ship(&mut commands, &mut meshes, &mut materials, &mut effects, Transform::from_xyz(20.0, 10.0, 20.0), AIPlayer);
}
