use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use common::card::CardCollection;
use currency::CurrencyPlugin;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub mod card_interactions;
pub mod currency;
pub mod net;
pub mod tilemap;
pub mod ui;
pub mod utils;

use card_interactions::CardInteractions;
use common::card::Card;
use net::packet_handler::PacketHandlerPlugin;
use tilemap::TilemapPlugin;
use ui::UiPlugin;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum GameState {
    Waiting,
    PreparingForGame,
    Playing,
}

#[derive(Copy, Clone, Debug, Resource)]
pub struct IsPlayer1(pub bool);

#[derive(Copy, Clone, Debug, Resource)]
pub struct IsSelfTurn(pub bool);

#[derive(Clone, Debug, Resource)]
pub struct Deck(pub Vec<Card>);

#[warn(unused_must_use)]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .insert_resource(CardCollection::new())
        .insert_resource(IsSelfTurn(false))
        .insert_resource(IsPlayer1(false))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 750.0,
                        height: 450.0,
                        title: "Multiplayer Game".to_string(),
                        present_mode: PresentMode::Fifo,
                        resizable: false,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(EguiPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(PacketHandlerPlugin)
        .add_plugin(CardInteractions)
        .add_plugin(CurrencyPlugin)
        .add_startup_system(spawn_camera)
        .add_state(GameState::Waiting)
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
