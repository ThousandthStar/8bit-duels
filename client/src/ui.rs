use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use kayak_ui::prelude::{widgets::*, *};

use crate::{
    card_interactions::{self, SelectedCardEntity, ViewingCardEntity},
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

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Waiting).with_system(waiting_ui))
            //            .add_plugin(KayakContextPlugin)
            //            .add_plugin(KayakWidgets)
            //            .add_startup_system_to_stage(StartupStage::PostStartup, startup)
            .add_system_set(
                SystemSet::on_update(GameState::PreparingForGame).with_system(preparing_ui),
            )
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(placing_troop))
            .insert_resource(DeckSelection {
                deck_selection: [UiCard::Skeleton; 5],
            })
            .insert_resource(CurrentlyPlacing(false))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(in_game_ui));
    }
}

/*
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
*/

#[derive(Debug, Clone, Copy, PartialEq)]
enum UiCard {
    Skeleton,
    Reaper,
}

impl UiCard {
    fn iter() -> [UiCard; 2] {
        [UiCard::Skeleton, UiCard::Reaper]
    }

    fn to_card(&self, card_collection: HashMap<String, Card>) -> Card {
        match self {
            UiCard::Skeleton => {
                return card_collection.get("skeleton").unwrap().clone();
            }
            UiCard::Reaper => {
                return card_collection.get("reaper").unwrap().clone();
            }
        }
    }
}

impl Display for UiCard {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, fmt)
    }
}

#[derive(Resource)]
struct DeckSelection {
    deck_selection: [UiCard; 5],
}

/// marker component for when a card is being placed
#[derive(Component)]
struct CurrentlyPlacingCard(Card);

#[derive(Resource)]
struct CurrentlyPlacing(bool);

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
                let card = ui_card.to_card(card_collection.0.clone());
                deck.push(card.clone());
            }
            let mut mutex_guard = queue_out.0.lock().unwrap();
            mutex_guard.push_back(ClientMessage::Deck(deck.clone()));
            commands.insert_resource(Deck(deck));
        }
    });
}

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
    deck: Res<Deck>,
    spirit_count: Res<Spirits>,
    pawn_count: Res<Pawns>,
    is_player_1: Res<IsPlayer1>,
    card_sprites: Res<CardSprites>,
    tile_size: Res<TileSize>,
    mut commands: Commands,
    mut is_placing: ResMut<CurrentlyPlacing>,
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
        ui.add_space(10.0);
        if let Some(card_entity) = selected_card.0.clone() {
            if card_entity.is_owned_by_p1() == is_player_1.0 {
                ui.monospace("Your troop");
            } else {
                ui.monospace("Your opponent's troop");
            }
            ui.monospace(format!("Current card: {}", card_entity.get_card().name));
            ui.monospace(format!("Attack: {}", card_entity.get_card().get_damage()));
            ui.monospace(format!("HP: {}", card_entity.current_hp));
        }
        ui.add_space(10.0);
        ui.monospace(format!("Pawns: {}", pawn_count.0));
        ui.monospace(format!("Spirits: {}", spirit_count.0));
        ui.add_space(10.0);
        ui.monospace("Spawn card");
        for card in &deck.0 {
            if ui.button(card.get_name()).clicked()
                && !is_placing.0
                && pawn_count.0 > 0
                && spirit_count.0 >= card.get_cost()
                && is_self_turn.0
            {
                is_placing.0 = true;
                let mut sprite =
                    TextureAtlasSprite::new(card_sprites.1.get(&card.get_name()).unwrap().clone());
                sprite.custom_size = Some(Vec2::splat(tile_size.0 * 0.8));

                commands
                    .spawn(SpriteSheetBundle {
                        sprite,
                        texture_atlas: card_sprites.0.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 500.0),
                        ..Default::default()
                    })
                    .insert(CurrentlyPlacingCard(card.clone()));
            }
        }
    });
}

fn placing_troop(
    mut queue_out: ResMut<QueueOut>,
    mut placing_query: Query<
        (Entity, Option<&CurrentlyPlacingCard>, &mut Transform),
        Without<Camera>,
    >,
    windows: Res<Windows>,
    cam_query: Query<(&Camera, &GlobalTransform)>,
    mouse: Res<Input<MouseButton>>,
    tile_size: Res<TileSize>,
    mut commands: Commands,
    mut is_placing: ResMut<CurrentlyPlacing>,
    mut pawn_count: ResMut<Pawns>,
    mut spirit_count: ResMut<Spirits>,
    is_player_1: Res<IsPlayer1>,
) {
    let (camera, global_transform) = cam_query.single();
    let window = windows.get_primary().unwrap();

    if let Some(pos) = window.cursor_position() {
        for (entity, option_placing_card, mut transform) in placing_query.iter_mut() {
            if let Some(currently_placing_card) = option_placing_card {
                let world_pos =
                    utils::screen_to_world_position(pos, &camera, &global_transform, &window);
                transform.translation.x = world_pos.x;
                transform.translation.y = world_pos.y;

                if mouse.just_pressed(MouseButton::Right) {
                    commands.entity(entity).despawn();
                    is_placing.0 = false;
                }

                if mouse.just_pressed(MouseButton::Left) {
                    let mut x = pos.x;
                    let mut y = pos.y;
                    if x < tile_size.0 * 5.0 || x > tile_size.0 * 10.0 {
                        return;
                    }
                    x -= tile_size.0 * 5.0;
                    x -= x % tile_size.0;
                    y -= y % tile_size.0;
                    x /= 50.0;
                    y /= 50.0;
                    if y > 3.0 {
                        return;
                    }
                    if is_player_1.0 {
                        y = 8.0 - y;
                    } else {
                        x = 4.0 - x;
                    }
                    println!("x: {}", x);
                    println!("y: {}", y);
                    queue_out
                        .0
                        .lock()
                        .unwrap()
                        .push_back(ClientMessage::SpawnCard(
                            currently_placing_card.0.clone(),
                            x as i32,
                            y as i32,
                        ));
                    commands.entity(entity).despawn();
                    is_placing.0 = false;
                    pawn_count.0 -= 1;
                    spirit_count.0 -= currently_placing_card.0.get_cost();
                    return;
                }
            }
        }
    }
}
