use bevy::render::texture::{ImageSampler, ImageSettings};
use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use game::card_interactions::CardInteractions;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub mod game;
pub mod net;
pub mod tilemap;
pub mod ui;

use game::card::{CardCollection, CardPlugin, CardSprites};
use net::packet_handler::PacketHandlerPlugin;
use tilemap::{TileSize, TilemapPlugin};
use ui::UiPlugin;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) enum GameState {
    Waiting,
    PreparingForGame,
    Playing,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct IsPlayer1(pub(crate) bool);

#[warn(unused_must_use)]
fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .insert_resource(ImageSettings {
            default_sampler: ImageSampler::nearest_descriptor(),
        })
        .insert_resource(WindowDescriptor {
            width: 750.0,
            height: 450.0,
            title: "Multiplayer Game".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(PacketHandlerPlugin)
        .add_plugin(CardPlugin)
        .add_plugin(CardInteractions)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_camera)
        .add_state(GameState::Waiting)
        .run();
}

fn test(mut commands: Commands, card_sprites: Res<CardSprites>, tile_size: Res<TileSize>) {
    let mut sprite = TextureAtlasSprite::new(card_sprites.1.get("skeleton").unwrap().clone());
    sprite.custom_size = Some(Vec2::splat(tile_size.0));
    let mut transform = Transform::from_xyz(0., 0., 500.);
    commands.spawn_bundle(SpriteSheetBundle {
        sprite,
        texture_atlas: card_sprites.0.clone(),
        transform,
        ..Default::default()
    });
}

#[derive(Component)]
struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    // spawn camera

    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::WindowSize;

    commands.spawn_bundle(camera).insert(MainCamera);
}
