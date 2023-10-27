use bevy::prelude::*;
use bevy_kira_audio::prelude::*;


pub struct SpaceShipSoundPlugin;

impl Plugin for SpaceShipSoundPlugin {
    fn build(&self, app: &mut App) {
        // app;
        app
        .add_plugins(AudioPlugin)
        .add_audio_channel::<MainThruster>()
        .add_audio_channel::<SideThruster>()
        .add_systems(Startup, play)
        .add_systems(Update, (play_main_thruster_sound, play_side_thruster_sound));
    }
}

fn play(
    main_thruster: Res<AudioChannel<MainThruster>>,
    side_thruster: Res<AudioChannel<SideThruster>>,
    asset_server: Res<AssetServer>,
) {
    main_thruster.play(asset_server.load("sounds/main_thruster.mp3")).looped();
    side_thruster.play(asset_server.load("sounds/side_thruster.mp3")).looped();
}


// pub fn setup_thruster_sounds(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
//     commands.spawn((
//         AudioBundle {
//             source: asset_server.load("sounds/main_thruster.mp3"),
//             ..default()
//         },
//         // MainThruster,
//     ));
//     commands.spawn((
//         AudioBundle {
//             source: asset_server.load("sounds/side_thruster.mp3"),
//             ..default()
//         },
//         SideThruster,
//     ));
    // audio.play(asset_server.load("sounds/main_thruster.mp3")).looped();
// }

#[derive(Resource)]
pub struct MainThruster;

#[derive(Resource)]
pub struct SideThruster;

pub fn play_main_thruster_sound(
    keyboard_input: Res<Input<KeyCode>>,
    main_thruster: Res<AudioChannel<MainThruster>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        main_thruster.resume();
    } else {
        main_thruster.pause();
    }
}

pub fn play_side_thruster_sound(
    keyboard_input: Res<Input<KeyCode>>,
    side_thruster: Res<AudioChannel<SideThruster>>,
) {
    if keyboard_input.any_pressed([KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D, KeyCode::X]) {
        side_thruster.resume();
    } else {
        side_thruster.pause();
    }
}
