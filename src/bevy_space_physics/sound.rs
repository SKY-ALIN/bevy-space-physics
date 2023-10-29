use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::ship_plugin::SpaceShip;

pub struct SpaceShipSoundPlugin;

impl Plugin for SpaceShipSoundPlugin {
    fn build(&self, app: &mut App) {
        // app;
        app
        .add_plugins(AudioPlugin)
        .add_audio_channel::<MainThruster>()
        .add_audio_channel::<SideThruster>()
        .add_systems(Startup, setup_thruster_sounds)
        .add_systems(Update, play_thruster_sounds);
    }
}

fn setup_thruster_sounds(
    main_thruster: Res<AudioChannel<MainThruster>>,
    side_thruster: Res<AudioChannel<SideThruster>>,
    asset_server: Res<AssetServer>,
) {
    main_thruster.play(asset_server.load("sounds/main_thruster.mp3")).looped();
    side_thruster.play(asset_server.load("sounds/side_thruster.mp3")).looped();
}

#[derive(Resource)]
pub struct MainThruster;

#[derive(Resource)]
pub struct SideThruster;

pub fn play_thruster_sounds(
    keyboard_input: Res<Input<KeyCode>>,
    ship_query: Query<&SpaceShip>,
    main_thruster: Res<AudioChannel<MainThruster>>,
    side_thruster: Res<AudioChannel<SideThruster>>,
) {
    let Ok(ship) = ship_query.get_single() else { return };
    let keys = ship.control_keys;
    if keyboard_input.pressed(keys.move_forward_key) {
        main_thruster.resume();
    } else {
        main_thruster.pause();
    }

    let side_thruster_keys = [
        keys.move_back_key,
        keys.move_up_key,
        keys.move_down_key,
        keys.move_left_key,
        keys.move_right_key,
    ];
    if keyboard_input.any_pressed(side_thruster_keys) {
        side_thruster.resume();
    } else {
        side_thruster.pause();
    }
}
