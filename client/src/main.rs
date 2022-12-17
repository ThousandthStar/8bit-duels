use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use common::card::CardCollection;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub(crate) mod card_interactions;
pub(crate) mod net;
pub(crate) mod tilemap;
pub(crate) mod ui;

use card_interactions::CardInteractions;
use net::packet_handler::PacketHandlerPlugin;
use tilemap::{CardSprites, TileSize, TilemapPlugin};
use ui::UiPlugin;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) enum GameState {
    Waiting,
    PreparingForGame,
    Playing,
}

#[derive(Copy, Clone, Debug, Resource)]
pub(crate) struct IsPlayer1(pub(crate) bool);

#[derive(Copy, Clone, Debug, Resource)]
pub(crate) struct IsSelfTurn(pub(crate) bool);

#[warn(unused_must_use)]
fn main() {
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
