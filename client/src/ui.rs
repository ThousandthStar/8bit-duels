use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{
    card_interactions::{self, is_in_boundary, SelectedCardEntity, ViewingCardEntity},
    currency::{Pawns, Spirits},
    net::{self, QueueOut},
    tilemap::{CardSprites, TileSize},
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

use in_game_ui::InGameUiPlugin;
use main_menu::MainMenuUiPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Waiting).with_system(waiting_ui))
            .add_plugin(MainMenuUiPlugin)
            .add_plugin(InGameUiPlugin);
    }
}

fn waiting_ui(
    mut context: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
) {
    egui::Window::new("Connect to server").show(context.ctx_mut(), |ui| {
        if ui.button("Connect").clicked() {
            net::init(&mut commands);
            state.set(GameState::PreparingForGame);
        }
    });
}
