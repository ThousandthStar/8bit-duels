use crate::DevMode;

use super::*;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

pub struct SettingsUiPlugin;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Settings).with_system(settings_ui))
            .add_system_set(SystemSet::on_enter(GameState::Settings).with_system(build_ui))
            .add_system_set(SystemSet::on_exit(GameState::Settings).with_system(destroy_ui))
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
