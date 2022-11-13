use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use serde_json::Value;

use crate::{
    card_interactions::SelectedCardEntity,
    net::{self, QueueOut},
    tilemap::TileSize,
    GameState,
};

use common::card::CardCollection;
use std::{fmt, fmt::Display};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Waiting).with_system(waiting_ui))
            .add_system_set(
                SystemSet::on_update(GameState::PreparingForGame).with_system(preparing_ui),
            )
            .insert_resource(DeckSelection {
                deck_selection: [UiCard::Skeleton; 10],
            })
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(in_game_ui));
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum UiCard {
    Skeleton,
    GoldMine,
}

impl UiCard {
    fn iter() -> [UiCard; 2] {
        [UiCard::Skeleton, UiCard::GoldMine]
    }
}

impl Display for UiCard {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, fmt)
    }
}

struct DeckSelection {
    deck_selection: [UiCard; 10],
}

fn preparing_ui(
    mut context: ResMut<EguiContext>,
    mut deck_selection: ResMut<DeckSelection>,
    mut queue_out: ResMut<QueueOut>,
    mut commands: Commands,
    card_collection: Res<CardCollection>,
) {
    egui::Window::new("Deck building").show(context.ctx_mut(), |ui| {
        ui.add_space(30.);
        ui.monospace(
            egui::RichText::new("Build your deck")
                .color(egui::Color32::YELLOW)
                .underline()
                .size(15.),
        );
        ui.add_space(10.);
        let mut i = 0;
        for card_selection in &mut deck_selection.deck_selection {
            egui::ComboBox::from_id_source(i)
                .selected_text(card_selection.to_string())
                .show_ui(ui, |ui| {
                    for card_option in UiCard::iter() {
                        ui.selectable_value(card_selection, card_option, card_option.to_string());
                    }
                });
            i += 1;
            ui.add_space(1.);
        }
        ui.add_space(9.);
        if ui.button("Ready").clicked() {
            let mut serialized_deck: Vec<Value> = vec![];
            for ui_card in deck_selection.deck_selection {
                let card = match ui_card {
                    UiCard::Skeleton => card_collection.0.get("skeleton").unwrap(),
                    UiCard::GoldMine => card_collection.0.get("gold-mine").unwrap(),
                };
                serialized_deck.push(serde_json::to_value(card).unwrap());
            }
            let mut json: Value = serde_json::from_str(
                r#"
                {
                    "packet-type": "player-deck"
                }
            "#,
            )
            .unwrap();
            if let Value::Object(ref mut map) = json {
                map.insert("deck".to_string(), Value::Array(serialized_deck));
            }
            let mut mutex_guard = queue_out.0.lock().unwrap();
            mutex_guard.push_back(json.to_string());
            drop(mutex_guard);
        }
    });
}

#[warn(unused_must_use)]
fn waiting_ui(
    mut context: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
) {
    egui::Window::new("Main Menu").show(context.ctx_mut(), |ui| {
        if ui.button("Play").clicked() {
            net::init(&mut commands);
            state.set(GameState::PreparingForGame);
        }
    });
}

fn in_game_ui(mut context: ResMut<EguiContext>, selected_card: Res<SelectedCardEntity>) {
    egui::Window::new("Playing").show(context.ctx_mut(), |ui| {
        ui.monospace("Playing the game!");
    });
}
