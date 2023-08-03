use belly::{prelude::*, widgets::input::button::BtnEvent};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{
    card_interactions::{ViewingCardEntity},
    currency::{Pawns, Spirits},
    net::{self, QueueOut},
    tilemap::{CardSprites, Tile, TileSize},
    utils, Deck, GameState, IsPlayer1, IsSelfTurn,
};

use common::{
    card::{Card},
    messages::ClientMessage,
};
use std::time::Duration;

pub mod before_game;
pub mod in_game_ui;
pub mod settings;

use before_game::BeforeGamePlugin;
use in_game_ui::InGameUiPlugin;
use settings::{Settings, SettingsUiPlugin};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Waiting).with_system(waiting_ui))
            .add_system_set(SystemSet::on_enter(GameState::Waiting).with_system(build_ui))
            .add_system_set(SystemSet::on_exit(GameState::Waiting).with_system(destroy_ui))
            .add_startup_system_to_stage(StartupStage::PostStartup, setup_main_menu)
            .add_system_set(
                SystemSet::on_update(GameState::Waiting)
                    .with_system(main_menu_buttons)
                    .after(show_main_menu_elements),
            )
            .add_system_set(SystemSet::on_exit(GameState::Waiting).with_system(remove_bg_image))
            .add_system_set(
                SystemSet::on_enter(GameState::Waiting).with_system(show_main_menu_elements),
            )
            .add_system_set(SystemSet::on_enter(GameState::Waiting).with_system(
                |mut show_menu: ResMut<ShowMenu>| {
                    show_menu.0 = false;
                },
            ))
            .add_startup_system(|mut commands: Commands, asset_server: Res<AssetServer>| {
                commands.insert_resource(GameFont(asset_server.load("Monocraft.otf")));
            })
            .add_startup_system(|mut commands: Commands| {
                commands.add(StyleSheet::load("stylesheet.ess"))
            })
            .insert_resource(ShowMenu(false))
            .add_plugin(EguiPlugin)
            .add_plugin(InGameUiPlugin)
            .add_plugin(BeforeGamePlugin)
            .add_plugin(BellyPlugin)
            .add_plugin(SettingsUiPlugin);
    }
}

#[derive(Component)]
pub struct UiBackgroundImage {
    frames: [Handle<Image>; 7],
    index: usize,
    timer: Timer,
}

#[derive(Resource)]
struct ShowMenu(bool);

#[derive(Resource)]
pub struct GameFont(pub Handle<Font>);

pub fn build_ui(mut tile_q: Query<&mut Visibility, With<Tile>>) {
    for mut visibility in tile_q.iter_mut() {
        visibility.is_visible = false;
    }
}

pub fn destroy_ui(
    mut commands: Commands,
    mut tile_q: Query<&mut Visibility, With<Tile>>,
    menu_query: Query<Entity, (With<Node>, Without<Tile>)>,
) {
    for mut visibility in tile_q.iter_mut() {
        visibility.is_visible = true;
    }
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn show_main_menu_elements(
    mut query_sprite: Query<&mut Visibility, With<UiBackgroundImage>>,
    mut commands: Commands,
    tile_size: Res<TileSize>,
) {
    let big_text_font_size = tile_size.0 / 2.0;
    let small_text_font_size = tile_size.0 / 4.0;
    commands.add(eml! {
        <div s:justify-content="center" c:start-text>
            <label s:font-size=big_text_font_size c:game-name-text value="8bit Duels"></label>
            <label s:font-size=small_text_font_size c:press-space-text value="Press Space!"></label>
        </div>
    });
    let mut visibility = query_sprite.single_mut();
    visibility.is_visible = true;
}

fn remove_bg_image(mut query: Query<&mut Visibility, With<UiBackgroundImage>>) {
    query.single_mut().is_visible = false;
}

fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tile_size: Res<TileSize>,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(tile_size.0 * 15.0, tile_size.0 * 9.0)),
                ..default()
            },
            texture: asset_server.load("main_menu0.png"),
            visibility: Visibility::INVISIBLE,
            ..default()
        })
        .insert(UiBackgroundImage {
            frames: [
                asset_server.load("main_menu0.png"),
                asset_server.load("main_menu1.png"),
                asset_server.load("main_menu2.png"),
                asset_server.load("main_menu3.png"),
                asset_server.load("main_menu4.png"),
                asset_server.load("main_menu5.png"),
                asset_server.load("main_menu6.png"),
            ],
            index: 0,
            timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
        });
}

fn waiting_ui(
    mut commands: Commands,
    time: Res<Time>,
    mut back_img_q: Query<(&mut Handle<Image>, &mut UiBackgroundImage)>,
    mut elements: Elements,
    mut show_menu: ResMut<ShowMenu>,
    tile_size: Res<TileSize>,
    keys: Res<Input<KeyCode>>,
) {
    let (mut img, mut img_settings) = back_img_q.single_mut();
    img_settings.timer.tick(time.delta() / 2);
    if img_settings.timer.finished() {
        img_settings.index += 1;
        if img_settings.index >= 7 {
            img_settings.index = 0;
        }
        let index = img_settings.index;
        *img.as_mut() = img_settings.frames[index].clone();
    }

    if keys.just_pressed(KeyCode::Space) && !show_menu.0 {
        show_menu.0 = true;
        let button_text_size = tile_size.0 / 2.5;
        let connection_error_text_size = tile_size.0 / 3.5;
        elements.select(".start-text").remove();
        commands.add(eml! {
            <body>
                <div c:mm-center-box>
                    <button c:mm-play-button id="play-button">
                        <img src="button.png" mode="fit">
                            <span s:font-size=button_text_size>"Play"</span>
                        </img>
                    </button>
                    <button c:mm-settings-button id="settings-button">
                        <img src="button.png" mode="fit">
                            <span s:font-size=button_text_size>"Settings"</span>
                        </img>
                    </button>
                    <label value="Could not connect to the server!"
                        s:font-size=connection_error_text_size
                        c:conn-err-text
                        c:hidden>
                    </label>
                </div>
            </body>
        });
    }
}

fn main_menu_buttons(
    mut state: ResMut<State<GameState>>,
    settings: Res<Settings>,
    mut commands: Commands,
    mut elements: Elements,
    mut reader: EventReader<BtnEvent>,
) {
    for event in reader.iter() {
        match *event {
            BtnEvent::Pressed(entity) => {
                if let Some(play_btn_ent) = elements.select("#play-button").entities().get(0) {
                    if play_btn_ent == &entity {
                        match net::init(&mut commands, &settings.server_addr) {
                            Ok(_) => {
                                state.set(GameState::PreparingForGame).unwrap();
                            }
                            Err(e) => {
                                elements.select(".conn-err-text").remove_class("hidden");
                                bevy::log::error!("Could not connect to the server: {}", e);
                            }
                        }
                    }
                }
                if let Some(settings_btn_ent) =
                    elements.select("#settings-button").entities().get(0)
                {
                    if settings_btn_ent == &entity {
                        state.set(GameState::Settings).unwrap();
                    }
                }
            }
            _ => {}
        }
    }
}
