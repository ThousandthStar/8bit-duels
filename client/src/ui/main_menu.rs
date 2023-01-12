use crate::net::QueueOut;
use crate::tilemap::Tile;
use crate::{Deck, GameState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use common::{card::*, messages::*};
use std::collections::HashMap;
use std::fmt::{self, Display};

pub struct MainMenuUiPlugin;

impl Plugin for MainMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::PreparingForGame).with_system(main_menu),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::PreparingForGame).with_system(super::build_ui),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::PreparingForGame).with_system(super::destroy_ui),
        )
        .insert_resource(DeckSelection::default());
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum UiCard {
    Skeleton,
    Reaper,
    Kraken,
    Spider,
}

impl UiCard {
    fn iter() -> [UiCard; 4] {
        [
            UiCard::Skeleton,
            UiCard::Reaper,
            UiCard::Kraken,
            UiCard::Spider,
        ]
    }

    fn to_card(&self, card_collection: HashMap<String, Card>) -> Card {
        return match self {
            UiCard::Skeleton => card_collection.get("skeleton").unwrap().clone(),
            UiCard::Reaper => card_collection.get("reaper").unwrap().clone(),
            UiCard::Kraken => card_collection.get("kraken").unwrap().clone(),
            UiCard::Spider => card_collection.get("spider").unwrap().clone(),
        };
    }
}

impl Display for UiCard {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, fmt)
    }
}

#[derive(Resource, Clone)]
struct DeckSelection {
    deck_selection: [UiCard; 5],
}

impl Default for DeckSelection {
    fn default() -> Self {
        Self {
            deck_selection: [
                UiCard::Skeleton,
                UiCard::Reaper,
                UiCard::Kraken,
                UiCard::Spider,
                UiCard::Skeleton,
            ],
        }
    }
}
fn main_menu(
    mut context: ResMut<EguiContext>,
    mut deck_selection: ResMut<DeckSelection>,
    mut queue_out: ResMut<QueueOut>,
    mut commands: Commands,
    card_collection: Res<CardCollection>,
) {
    egui::Window::new("Main Menu").show(context.ctx_mut(), |ui| {
        ui.add_space(30.);
        ui.monospace(
            egui::RichText::new("Build your deck")
                .color(egui::Color32::YELLOW)
                .underline()
                .size(15.),
        );
        ui.add_space(10.);
        let mut i = 0;
        let deck_selection_clone = deck_selection.clone();
        for card_selection in &mut deck_selection.deck_selection {
            egui::ComboBox::from_id_source(i)
                .selected_text(card_selection.to_string())
                .show_ui(ui, |ui| {
                    for card_option in UiCard::iter() {
                        if !deck_selection_clone.deck_selection.contains(&card_option) {
                            ui.selectable_value(
                                card_selection,
                                card_option,
                                card_option.to_string(),
                            );
                        }
                    }
                });
            i += 1;
            ui.add_space(1.);
        }
        ui.add_space(9.);
        if ui.button("Ready").clicked() {
            let mut deck: Vec<Card> = vec![];
            for ui_card in deck_selection.deck_selection {
                let card = ui_card.to_card(card_collection.0.clone());
                deck.push(card.clone());
            }
            let mut mutex_guard = queue_out.0.lock().unwrap();
            mutex_guard.push_back(ClientMessage::PlayerInfo("player".to_owned(), deck.clone()));
            commands.insert_resource(Deck(deck));
        }
    });
}
