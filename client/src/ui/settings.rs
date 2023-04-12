use crate::DevMode;

use super::{widgets::SwitchTextures, *};
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

pub struct SettingsUiPlugin;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Settings).with_system(settings_ui))
            .add_system_set(SystemSet::on_enter(GameState::Settings).with_system(build_ui))
            .add_system_set(SystemSet::on_exit(GameState::Settings).with_system(destroy_ui))
            .add_system_set(SystemSet::on_enter(GameState::Settings).with_system(setup_settings_ui))
            .add_startup_system_to_stage(StartupStage::PreStartup, pre_setup)
            .add_startup_system_to_stage(StartupStage::Startup, setup)
            .insert_resource(Settings::default());
    }
}

#[derive(Serialize, Deserialize, Clone, Resource)]
pub struct Settings {
    pub username: String,
    pub server_addr: String,
    pub debug_mode: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            username: "Player".to_owned(),
            server_addr: "127.0.0.1:1000".to_owned(),
            debug_mode: false,
        }
    }
}

fn pre_setup(mut commands: Commands, dev_mode: Res<DevMode>) {
    if !dev_mode.0 {
        commands.insert_resource(PkvStore::new("ThousandthStar", "8bit Duels"));
    }
}

fn setup(mut settings: ResMut<Settings>, pkv: Option<Res<PkvStore>>) {
    if pkv.is_none() {
        return;
    }
    let pkv = pkv.unwrap();
    if let Ok(stored_settings) = pkv.get::<Settings>("settings") {
        *settings.into_inner() = stored_settings;
    }
}

fn setup_settings_ui(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    tile_size: Res<TileSize>,
    switch_textures: Res<SwitchTextures>,
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("ui_bg.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(tile_size.0 * 15.0, tile_size.0 * 9.0)),
            ..default()
        },
        ..default()
    });
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
                            font: asset_server.load("Monocraft.otf"),
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
                                font: asset_server.load("Monocraft.otf"),
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
                                    bottom: Val::Px(0.),
                                    left: Val::Px(66.0),
                                    ..default()
                                },
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                position_type: PositionType::Absolute,
                                size: Size::new(Val::Px(288.0), Val::Px(72.0)),
                                ..default()
                            },
                            image: asset_server.load("button.png").into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Save",
                                    TextStyle {
                                        font: asset_server.load("Monocraft.otf"),
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
                            font: asset_server.load("Monocraft.otf"),
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
                                font: asset_server.load("Monocraft.otf"),
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
                            size: Size::new(Val::Px(128.0), Val::Px(64.0)),
                            ..default()
                        },
                        image: switch_textures.off.clone_weak().into(),
                        ..default()
                    });
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                position: UiRect {
                                    bottom: Val::Px(0.),
                                    left: Val::Px(66.0),
                                    ..default()
                                },
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                position_type: PositionType::Absolute,
                                size: Size::new(Val::Px(288.0), Val::Px(72.0)),
                                ..default()
                            },
                            image: asset_server.load("button.png").into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Back",
                                    TextStyle {
                                        font: asset_server.load("Monocraft.otf"),
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
    mut context: ResMut<EguiContext>,
    mut pkv: Option<ResMut<PkvStore>>,
    mut settings: ResMut<Settings>,
    mut state: ResMut<State<GameState>>,
) {
    if pkv.is_none() {
        return;
    }
    let mut pkv = pkv.unwrap();
    egui::SidePanel::left("settings_panel")
        .frame(egui::Frame::none())
        .show(context.ctx_mut(), |ui| {
            ui.vertical(|ui| {
                ui.label("Settings Menu");
                ui.horizontal(|ui| {
                    ui.label("Username");
                    ui.text_edit_singleline(&mut settings.username);
                });
                ui.horizontal(|ui| {
                    ui.label("Server address");
                    ui.text_edit_singleline(&mut settings.server_addr);
                });
                if ui
                    .button(if settings.debug_mode {
                        "Disable debug mode"
                    } else {
                        "Enable debug mode"
                    })
                    .clicked()
                {
                    settings.debug_mode = !settings.debug_mode;
                }
                if ui.button("Save").clicked() {
                    pkv.set::<Settings>("settings", &settings.clone().into());
                }
                if ui.button("Back").clicked() {
                    state.set(GameState::Waiting);
                }
            });
        });
}
