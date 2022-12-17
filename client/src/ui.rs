use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use kayak_ui::prelude::{widgets::*, *};
use serde_json::Value;

use crate::{
    card_interactions::{SelectedCardEntity, ViewingCardEntity},
    net::{self, QueueOut},
    tilemap::TileSize,
    GameState, IsSelfTurn,
};

use common::{
    card::{Card, CardCollection},
    messages::ClientMessage,
};
use std::{fmt, fmt::Display};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Waiting).with_system(waiting_ui))
            .add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)
            .add_startup_system_to_stage(StartupStage::PostStartup, startup)
            .add_system_set(
                SystemSet::on_update(GameState::PreparingForGame).with_system(preparing_ui),
            )
            .insert_resource(DeckSelection {
                deck_selection: [UiCard::Skeleton; 10],
            })
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(in_game_ui));
    }
}

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    tile_size: Res<TileSize>,
    asset_server: Res<AssetServer>,
    selected_card_entity: Res<SelectedCardEntity>,
) {
    font_mapping.set_default(asset_server.load("Monocraft.kayak_font"));
    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    let parent_id = None;
    let ui_bg_image = asset_server.load("ui_background.png");
    let ui_bg_image_2 = asset_server.load("ui_background.png");
    rsx! {
        <KayakAppBundle>
            <NinePatchBundle>
                    <NinePatchBundle
                        nine_patch = {NinePatch{
                            handle: ui_bg_image,
                            ..Default::default()
                        }}
                        styles = {KStyle{
                            width: StyleProp::Value(Units::Pixels(tile_size.0 * 5.0)),
                            height: StyleProp::Value(Units::Pixels(tile_size.0 * 9.0)),
                            ..KStyle::default()
                        }}
                    >
                        <TextWidgetBundle
                            text={TextProps {
                                content: "8bit Duels".into(),
                                size: 20.0,
                                user_styles: KStyle{
                                    color: StyleProp::Value(Color::hex("a05e5e").unwrap_or(Color::BLACK)),
                                    offset: StyleProp::Value(Edge::axis(Units::Pixels(tile_size.0 * 1.35), Units::Pixels(tile_size.0))),
                                    ..KStyle::default()
                                },
                                ..Default::default()
                            }}
                        />
                    </NinePatchBundle>

                    <NinePatchBundle
                        nine_patch = {NinePatch{
                            handle: ui_bg_image_2,
                            ..Default::default()
                        }}
                        styles = {KStyle{
                            width: StyleProp::Value(Units::Pixels(tile_size.0 * 5.0)),
                            height: StyleProp::Value(Units::Pixels(tile_size.0 * 9.0)),
                            left: StyleProp::Value(Units::Pixels(tile_size.0 * 10.0)),
                            offset: StyleProp::Value(Edge::new(
                                    Units::Pixels(tile_size.0 * -9.0),
                                    Units::Auto,
                                    Units::Auto,
                                    Units::Auto
                            )),
                            ..KStyle::default()
                        }}
                    />
            </NinePatchBundle>
        </KayakAppBundle>
    }

    commands.spawn(UICameraBundle::new(widget_context));
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

#[derive(Resource)]
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
            let mut deck: Vec<Card> = vec![];
            for ui_card in deck_selection.deck_selection {
                let card = match ui_card {
                    UiCard::Skeleton => card_collection.0.get("skeleton").unwrap(),
                    UiCard::GoldMine => card_collection.0.get("gold-mine").unwrap(),
                };
                deck.push(card.clone());
            }
            let mut mutex_guard = queue_out.0.lock().unwrap();
            mutex_guard.push_back(ClientMessage::Deck(deck));
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

fn in_game_ui(
    mut context: ResMut<EguiContext>,
    selected_card: Res<ViewingCardEntity>,
    mut queue_out: ResMut<QueueOut>,
    mut is_self_turn: ResMut<IsSelfTurn>,
) {
    egui::Window::new("Playing").show(context.ctx_mut(), |ui| {
        if is_self_turn.0 {
            ui.label("Your turn!");
            if ui.button("End turn").clicked() {
                queue_out
                    .0
                    .lock()
                    .unwrap()
                    .push_back(ClientMessage::EndTurn);
                is_self_turn.0 = false;
            }
        } else {
            ui.label("Opponent's turn!");
        }
        if let Some(card_entity) = selected_card.0.clone() {
            ui.monospace(format!("Current card: {}", card_entity.get_card().name));
            ui.monospace(format!("Attack: {}", card_entity.get_card().get_damage()));
            ui.monospace(format!("HP: {}", card_entity.current_hp));
        }
    });
}
