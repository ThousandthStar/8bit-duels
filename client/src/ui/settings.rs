use bevy::log::{error, info};
use std::{error::Error, fs, path::Path};

use crate::DevMode;

use super::{
    widgets::{Switch, SwitchTextures},
    *,
};
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

pub struct SettingsUiPlugin;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        let mut settings: Settings = Settings::default();
        match load_settings_from_file() {
            Ok(loaded_settings) => {
                settings = loaded_settings;
            }
            Err(e) => {
                error!("Critical error loading config: {}", e);
                info!("Trying to write default settings to file");
                match write_settings_to_file(&Settings::default()) {
                    Ok(_) => {
                        info!("Successfully wrote default settings to config file");
                    }
                    Err(e) => {
                        error!("Failed to write: {}", e);
                    }
                }
            }
        }
        app.add_system_set(SystemSet::on_update(GameState::Settings).with_system(settings_ui))
            .add_system_set(SystemSet::on_enter(GameState::Settings).with_system(build_ui))
            .add_system_set(SystemSet::on_exit(GameState::Settings).with_system(destroy_ui))
            .add_system_set(SystemSet::on_exit(GameState::Settings).with_system(remove_bg_image))
            .add_system_set(SystemSet::on_enter(GameState::Settings).with_system(setup_settings_ui))
            .add_startup_system_to_stage(StartupStage::Startup, update_window_scale)
            .insert_resource(DevMode(settings.debug_mode))
            .add_state(if settings.debug_mode {
                GameState::Waiting
            } else {
                GameState::Opening
            })
            .insert_resource(settings);
    }
}

#[derive(Serialize, Deserialize, Clone, Resource, Debug)]
pub struct Settings {
    pub username: String,
    pub server_addr: String,
    pub debug_mode: bool,
    pub volume: u8,
    pub window_scale: u8,
    pub deck: Vec<Card>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            username: "Player".to_owned(),
            server_addr: "127.0.0.1:1000".to_owned(),
            debug_mode: false,
            volume: 100,
            window_scale: 4,
            deck: vec![
                "skeleton".into(),
                "reaper".into(),
                "kraken".into(),
                "spider".into(),
                "skeleton".into(),
            ],
        }
    }
}

fn load_settings_from_file() -> Result<Settings, Box<dyn Error>> {
    let raw_config = fs::read_to_string(Path::new("assets").join("config.ron"))?;
    let settings: Settings = ron::from_str(&raw_config)?;
    Ok(settings)
}

fn write_settings_to_file(settings: &Settings) -> Result<(), Box<dyn Error>> {
    let stringed = ron::to_string(settings)?;
    fs::write(Path::new("assets").join("config.ron"), stringed.as_str())?;
    Ok(())
}

pub fn update_window_scale(mut windows: ResMut<Windows>, settings: Res<Settings>) {
    let mut window = windows.get_primary_mut().unwrap();
    window.set_resolution(
        300.0 * settings.window_scale as f32,
        180.0 * settings.window_scale as f32,
    );
}
// marker components
#[derive(Component)]
struct BackButton;
#[derive(Component)]
struct SaveButton;
#[derive(Component)]
struct BackgroundImage;
#[derive(Component)]
struct SavedInfo;

fn remove_bg_image(mut commands: Commands, query: Query<Entity, With<BackgroundImage>>) {
    commands.entity(query.single()).despawn();
}

fn setup_settings_ui(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    tile_size: Res<TileSize>,
    switch_textures: Res<SwitchTextures>,
    game_font: Res<GameFont>,
    settings: Res<Settings>,
) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("ui_bg.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(tile_size.0 * 15.0, tile_size.0 * 9.0)),
                ..default()
            },
            ..default()
        })
        .insert(BackgroundImage);
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position: UiRect {
                            left: Val::Percent(10.0),
                            top: Val::Percent(5.0),
                            ..default()
                        },
                        size: Size::new(Val::Percent(35.0), Val::Percent(90.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Username",
                        TextStyle {
                            font: game_font.0.clone_weak(),
                            font_size: tile_size.0 / 4.5,
                            color: Color::BLACK,
                        },
                    ));
                    parent.spawn(ButtonBundle {
                        style: Style {
                            position: UiRect {
                                top: Val::Percent(5.0),
                                ..default()
                            },
                            position_type: PositionType::Absolute,
                            size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                            ..default()
                        },
                        image: asset_server.load("text_box_bg.png").into(),
                        ..default()
                    });
                    parent.spawn(
                        TextBundle::from_section(
                            "Server Address",
                            TextStyle {
                                font_size: tile_size.0 / 4.5,
                                font: game_font.0.clone_weak(),
                                color: Color::BLACK,
                            },
                        )
                        .with_style(Style {
                            position: UiRect {
                                top: Val::Percent(17.5),
                                ..default()
                            },
                            position_type: PositionType::Absolute,
                            ..default()
                        }),
                    );
                    parent.spawn(ButtonBundle {
                        style: Style {
                            position: UiRect {
                                top: Val::Percent(22.0),
                                ..default()
                            },
                            position_type: PositionType::Absolute,
                            size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                            ..default()
                        },
                        image: asset_server.load("text_box_bg.png").into(),
                        ..default()
                    });
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                position: UiRect {
                                    bottom: Val::Percent(5.0),
                                    left: Val::Px(tile_size.0 * 0.825),
                                    ..default()
                                },
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                position_type: PositionType::Absolute,
                                size: Size::new(
                                    Val::Px(tile_size.0 * 3.6),
                                    Val::Px(tile_size.0 * 0.9),
                                ),
                                ..default()
                            },
                            image: asset_server.load("button.png").into(),
                            ..default()
                        })
                        .insert(SaveButton)
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Save",
                                    TextStyle {
                                        font: game_font.0.clone_weak(),
                                        font_size: tile_size.0 / 4.5,
                                        color: Color::WHITE.into(),
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect { ..default() },
                                    ..default()
                                })
                                .with_text_alignment(TextAlignment::CENTER),
                            );
                        });
                    parent
                        .spawn(
                            TextBundle::from_section(
                                "",
                                TextStyle {
                                    color: Color::RED.into(),
                                    font_size: tile_size.0 / 4.5,
                                    font: game_font.0.clone_weak(),
                                },
                            )
                            .with_style(Style {
                                position: UiRect {
                                    bottom: Val::Px(0.0),
                                    ..default()
                                },
                                position_type: PositionType::Absolute,
                                ..default()
                            }),
                        )
                        .insert(SavedInfo);
                });
            /*
             * Second column
             */
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position: UiRect {
                            left: Val::Percent(55.0),
                            top: Val::Percent(5.0),
                            ..default()
                        },
                        position_type: PositionType::Absolute,
                        size: Size::new(Val::Percent(35.0), Val::Percent(90.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Volume (0-100)",
                        TextStyle {
                            font: game_font.0.clone_weak(),
                            font_size: tile_size.0 / 4.5,
                            color: Color::BLACK,
                        },
                    ));
                    parent.spawn(ButtonBundle {
                        style: Style {
                            position: UiRect {
                                top: Val::Percent(5.0),
                                ..default()
                            },
                            position_type: PositionType::Absolute,
                            size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                            ..default()
                        },
                        image: asset_server.load("text_box_bg.png").into(),
                        ..default()
                    });
                    parent.spawn(
                        TextBundle::from_section(
                            "Debug Mode",
                            TextStyle {
                                font_size: tile_size.0 / 4.5,
                                font: game_font.0.clone_weak(),
                                color: Color::BLACK,
                            },
                        )
                        .with_style(Style {
                            position: UiRect {
                                top: Val::Percent(17.5),
                                ..default()
                            },
                            position_type: PositionType::Absolute,
                            ..default()
                        }),
                    );
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                position: UiRect {
                                    top: Val::Percent(22.0),
                                    ..default()
                                },
                                position_type: PositionType::Absolute,
                                size: Size::new(
                                    Val::Px(tile_size.0 * 1.6),
                                    Val::Px(tile_size.0 * 0.8),
                                ),
                                ..default()
                            },
                            image: switch_textures.off.clone_weak().into(),
                            ..default()
                        })
                        .insert(Switch {
                            on: settings.debug_mode,
                        });
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                position: UiRect {
                                    bottom: Val::Percent(5.0),
                                    left: Val::Px(tile_size.0 * 0.825),
                                    ..default()
                                },
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                position_type: PositionType::Absolute,
                                size: Size::new(
                                    Val::Px(tile_size.0 * 3.6),
                                    Val::Px(tile_size.0 * 0.9),
                                ),
                                ..default()
                            },
                            image: asset_server.load("button.png").into(),
                            ..default()
                        })
                        .insert(BackButton)
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Back",
                                    TextStyle {
                                        font: game_font.0.clone_weak(),
                                        font_size: tile_size.0 / 4.5,
                                        color: Color::WHITE.into(),
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect { ..default() },
                                    ..default()
                                })
                                .with_text_alignment(TextAlignment::CENTER),
                            );
                        });
                });
        });
}

fn settings_ui(
    mut settings: ResMut<Settings>,
    mut state: ResMut<State<GameState>>,
    button_q: Query<
        (&Interaction, Option<&BackButton>, Option<&SaveButton>),
        (Changed<Interaction>, Without<Switch>, Without<SavedInfo>),
    >,
    switch_q: Query<&Switch, Without<SavedInfo>>,
    mut saved_text_info_q: Query<&mut Text, With<SavedInfo>>,
) {
    for (interaction, back_btn_opt, save_btn_opt) in button_q.iter() {
        match *interaction {
            Interaction::Clicked => {
                if back_btn_opt.is_some() {
                    state.set(GameState::Waiting);
                }
                if save_btn_opt.is_some() {
                    settings.debug_mode = switch_q.single().on;
                    match write_settings_to_file(&settings) {
                        Ok(_) => {
                            saved_text_info_q.single_mut().sections[0].value =
                                "Settings Saved Successfully".to_owned();
                        }
                        Err(e) => {
                            saved_text_info_q.single_mut().sections[0].value =
                                "Failed to Save Settings".to_owned();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
