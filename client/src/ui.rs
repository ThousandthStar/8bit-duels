use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{
    card_interactions::{self, is_in_boundary, SelectedCardEntity, ViewingCardEntity},
    currency::{Pawns, Spirits},
    net::{self, QueueOut},
    tilemap::{CardSprites, Tile, TileSize},
    utils, Deck, GameState, IsPlayer1, IsSelfTurn,
};

use common::{
    card::{Card, CardCollection},
    messages::ClientMessage,
};
use std::{collections::HashMap, fmt, fmt::Display};

pub mod config;
pub mod in_game_ui;
pub mod main_menu;
pub mod settings;

use in_game_ui::InGameUiPlugin;
use main_menu::MainMenuUiPlugin;
use settings::{Settings, SettingsUiPlugin};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Waiting).with_system(waiting_ui))
            .add_system_set(SystemSet::on_enter(GameState::Waiting).with_system(build_ui))
            .add_system_set(SystemSet::on_exit(GameState::Waiting).with_system(destroy_ui))
            .add_plugin(MainMenuUiPlugin)
            .add_plugin(InGameUiPlugin)
            .add_plugin(SettingsUiPlugin);
    }
}

#[derive(Component)]
pub struct UiBackgroundImage;

pub fn build_ui(
    mut tile_q: Query<&mut Visibility, With<Tile>>,
    mut commands: Commands,
    tile_size: Res<TileSize>,
) {
    for mut visibility in tile_q.iter_mut() {
        visibility.is_visible = false;
    }
}

pub fn destroy_ui(mut tile_q: Query<&mut Visibility, With<Tile>>) {
    for mut visibility in tile_q.iter_mut() {
        visibility.is_visible = true;
    }
}

fn waiting_ui(
    mut context: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
    settings: Res<Settings>,
) {
    egui::Window::new("Connect to server").show(context.ctx_mut(), |ui| {
        if ui.button("Settings").clicked() {
            state.set(GameState::Settings);
        }
        if ui.button("Connect").clicked() {
            net::init(&mut commands, &settings.server_addr);
            state.set(GameState::PreparingForGame);
        }
    });
}
