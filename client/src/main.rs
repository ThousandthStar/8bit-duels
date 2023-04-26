use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use bevy_egui::EguiPlugin;
use bevy_kira_audio::AudioPlugin;
use common::card::CardCollection;
use currency::CurrencyPlugin;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

pub mod animations;
pub mod audio;
pub mod card_interactions;
pub mod currency;
pub mod net;
pub mod opening;
pub mod ownership_indicator;
pub mod stun_indicator;
pub mod tilemap;
pub mod ui;
pub mod utils;
pub mod debug;

use animations::AnimationPlugin;
use audio::GameAudioPlugin;
use card_interactions::CardInteractions;
use common::card::Card;
use net::packet_handler::PacketHandlerPlugin;
use opening::OpeningPlugin;
use ownership_indicator::OwnershipIndicatorPlugin;
use stun_indicator::StunIndicatorPlugin;
use tilemap::TilemapPlugin;
use ui::UiPlugin;
use debug::DebugPlugin;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum GameState {
    Opening,
    Waiting,
    PreparingForGame,
    Settings,
    DeckBuilding,
    MainMenu,
    Playing,
}

#[derive(Copy, Clone, Debug, Resource)]
pub struct IsPlayer1(pub bool);

#[derive(Copy, Clone, Debug, Resource)]
pub struct IsSelfTurn(pub bool);

#[derive(Clone, Debug, Resource)]
pub struct Deck(pub Vec<Card>);

#[derive(Resource)]
pub struct DevMode(bool);

#[warn(unused_must_use)]
pub fn main() {
    App::new()
        .insert_resource(CardCollection::new())
        .insert_resource(IsSelfTurn(false))
        .insert_resource(IsPlayer1(false))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 300.0 * 4.0,
                        height: 180.0 * 4.0,
                        title: "8bit Duels".to_string(),
                        present_mode: PresentMode::Fifo,
                        resizable: false,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(EguiPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(PacketHandlerPlugin)
        .add_plugin(CardInteractions)
        .add_plugin(CurrencyPlugin)
        .add_plugin(StunIndicatorPlugin)
        .add_plugin(OwnershipIndicatorPlugin)
        .add_plugin(GameAudioPlugin)
        .add_plugin(OpeningPlugin)
        .add_plugin(DebugPlugin)
        .add_startup_system(spawn_camera)
        .add_plugin(AnimationPlugin)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    // spawn camera

    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::WindowSize;

    commands.spawn_bundle(camera).insert(MainCamera);
}

#[cfg(test)]
mod tests {

    #[test]
    fn or_test() {
        assert!(true || false);
        assert!(true || true);
    }
}
