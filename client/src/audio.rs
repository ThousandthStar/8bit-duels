use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl, AudioSource};

use crate::{DevMode, GameState};

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<MenuMusicChannel>()
            .add_startup_system_to_stage(StartupStage::PreStartup, load_music)
            .add_system_set(SystemSet::on_enter(GameState::Waiting).with_system(main_menu_audio))
            .insert_resource(RestartGameAudio(true));
    }
}

#[derive(Resource)]
struct RestartGameAudio(bool);

#[derive(Resource)]
struct MenuMusicChannel;

#[derive(Resource)]
struct GameAudioAssets {
    menu_music: Handle<AudioSource>,
}

fn load_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    let menu_music = asset_server.load("main_menu.mp3");

    commands.insert_resource(GameAudioAssets { menu_music });
}

fn main_menu_audio(
    audio_assets: Res<GameAudioAssets>,
    channel: Res<AudioChannel<MenuMusicChannel>>,
    mut restart_game_audio: ResMut<RestartGameAudio>,
    dev_mode: Res<DevMode>,
) {
    if restart_game_audio.0 {
        channel
            .play(audio_assets.menu_music.clone_weak())
            .looped()
            .with_volume(if dev_mode.0 { 0.0 } else { 1.0 });
        restart_game_audio.0 = false;
    }
}
