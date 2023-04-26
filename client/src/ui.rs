use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_egui::{egui, EguiContext};
use bevy_kira_audio::prelude::*;

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
use std::time::Duration;

pub mod in_game_ui;
pub mod settings;
pub mod widgets;
pub mod before_game;

use in_game_ui::InGameUiPlugin;
use settings::{Settings, SettingsUiPlugin};
use widgets::WidgetPlugin;
use before_game::BeforeGamePlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Waiting).with_system(waiting_ui))
            .add_system_set(SystemSet::on_enter(GameState::Waiting).with_system(build_ui))
            .add_system_set(SystemSet::on_exit(GameState::Waiting).with_system(destroy_ui))
            .add_startup_system_to_stage(StartupStage::PostStartup, setup_main_menu)
            .add_system_set(SystemSet::on_update(GameState::Waiting).with_system(main_menu_buttons))
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
            .insert_resource(ShowMenu(false))
            .add_plugin(InGameUiPlugin)
            .add_plugin(WidgetPlugin)
            .add_plugin(BeforeGamePlugin)
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
    asset_server: Res<AssetServer>,
    tile_size: Res<TileSize>,
    game_font: Res<GameFont>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section(
                        "8bit Duels",
                        TextStyle {
                            font: game_font.0.clone_weak(),
                            font_size: tile_size.0 / 2.0,
                            color: Color::BLACK,
                        },
                    )
                    .with_style(Style {
                        position: UiRect::new(
                            Val::Percent(5.0),
                            Val::Auto,
                            Val::Percent(2.0),
                            Val::Auto,
                        ),
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        ..default()
                    }),
                )
                .insert(StartIndicator)
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Press Space!",
                            TextStyle {
                                font: game_font.0.clone_weak(),
                                font_size: tile_size.0 / 4.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_style(Style {
                            position: UiRect::new(
                                Val::Auto,
                                Val::Auto,
                                Val::Px(tile_size.0 / 1.5),
                                Val::Auto,
                            ),
                            ..default()
                        }),
                    );
                });
        });

    let mut visibility = query_sprite.single_mut();
    visibility.is_visible = true;
}

fn remove_bg_image(mut query: Query<&mut Visibility, With<UiBackgroundImage>>) {
    query.single_mut().is_visible = false;
}

//marker components
#[derive(Component)]
struct StartIndicator;
#[derive(Component)]
struct PlayButton;
#[derive(Component)]
struct SettingsButton;

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
    mut start_indicator_q: Query<Entity, With<StartIndicator>>,
    mut show_menu: ResMut<ShowMenu>,
    tile_size: Res<TileSize>,
    keys: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    game_font: Res<GameFont>,
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
        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(40.0), Val::Percent(45.0)),
                    position: UiRect::new(
                        Val::Percent(30.0),
                        Val::Auto,
                        Val::Percent(27.5),
                        Val::Auto,
                    ),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            position: UiRect::new(
                                Val::Percent(10.),
                                Val::Auto,
                                Val::Percent(10.),
                                Val::Auto,
                            ),
                            align_content: AlignContent::Center,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            position_type: PositionType::Absolute,
                            size: Size::new(Val::Percent(80.), Val::Percent(30.0)),
                            ..default()
                        },
                        image: UiImage(asset_server.load("button.png")),
                        ..default()
                    })
                    .insert(PlayButton)
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Play",
                            TextStyle {
                                font: game_font.0.clone_weak(),
                                font_size: tile_size.0 / 2.5,
                                color: Color::WHITE,
                            },
                        ));
                    });
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            position: UiRect::new(
                                Val::Percent(10.),
                                Val::Auto,
                                Val::Percent(60.),
                                Val::Auto,
                            ),
                            align_content: AlignContent::Center,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            position_type: PositionType::Absolute,
                            size: Size::new(Val::Percent(80.), Val::Percent(30.0)),
                            ..default()
                        },
                        image: UiImage(asset_server.load("button.png")),
                        ..default()
                    })
                    .insert(SettingsButton)
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Settings",
                            TextStyle {
                                font: game_font.0.clone_weak(),
                                font_size: tile_size.0 / 2.5,
                                color: Color::WHITE,
                            },
                        ));
                    });
            });
        commands
            .entity(start_indicator_q.single())
            .despawn_recursive();
    }
}

fn main_menu_buttons(
    mut state: ResMut<State<GameState>>,
    mut query: Query<(
        &Interaction,
        &mut BackgroundColor,
        Option<&PlayButton>,
        Option<&SettingsButton>,
    )>,
    settings: Res<Settings>,
    mut commands: Commands,
) {
    for (interaction, mut color, play_btn_opt, settings_btn_opt) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if play_btn_opt.is_some() {
                    net::init(&mut commands, &settings.server_addr);
                    state.set(GameState::PreparingForGame);
                }
                if settings_btn_opt.is_some() {
                    state.set(GameState::Settings);
                }
            }
            _ => {}
        }
    }
}
