use bevy::log::{error, info};
use std::{error::Error, fs, path::Path};

use crate::DevMode;

use super::*;

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
        app.add_system_set(
            SystemSet::on_update(GameState::Settings)
                .with_system(settings_ui)
                .after(setup_settings_ui),
        )
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
        });
        if settings.debug_mode {
            app.add_plugin(WorldInspectorPlugin);
        }
        app.insert_resource(settings);
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
                "crow".into(),
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
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(
        300.0 * settings.window_scale as f32,
        180.0 * settings.window_scale as f32,
    );
}
// marker components
#[derive(Component, Default)]
struct BackgroundImage;
#[derive(Component, Default)]
struct UsernameTextBox;
#[derive(Component, Default)]
struct ServerAddressTextBox;

fn remove_bg_image(mut commands: Commands, query: Query<Entity, With<BackgroundImage>>) {
    commands.entity(query.single()).despawn();
}

#[derive(Component, Default)]
struct Switch {
    texture: Handle<Image>,
    on: bool,
}

fn setup_settings_ui(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    tile_size: Res<TileSize>,
    settings: Res<Settings>,
) {
    let text_size = tile_size.0 / 4.5;
    let tile_size = tile_size.0;
    let button_handle: Handle<Image> = asset_server.load("button.png");
    let switch_handle_on: Handle<Image> = asset_server.load("switch_on.png");
    let switch_handle_off: Handle<Image> = asset_server.load("switch_off.png");
    let switch_comp = Switch {
        texture: if settings.debug_mode {
            switch_handle_on.clone_weak()
        } else {
            switch_handle_off.clone_weak()
        },
        on: settings.debug_mode,
    };
    let switch = commands.spawn_empty().id();
    let switch_img = commands.spawn_empty().id();
    let username = settings.username.clone();
    let server_addr = settings.server_addr.clone();
    commands.add(eml! {
        <body>
            <img src="ui_bg.png" mode="fit" with=BackgroundImage>
                <div c:s-container>
                    //
                    //left column
                    //
                    <div c:s-left-column>
                        // Username text
                        <label
                            value="Username"
                            s:font-size=text_size
                            c:s-top-text>
                        </label>
                        // Username input text box
                        <img
                            src="text_box_bg.png"
                            mode="fit"
                            c:s-username-tb-img>
                            <textinput
                                s:font-size=text_size
                                value=username
                                with=UsernameTextBox>
                            </textinput>
                        </img>
                        // Server Address text
                        <label
                            value="Server Address"
                            s:font-size=text_size
                            c:s-second-text>
                        </label>
                        // Server address text box
                        <img
                            src="text_box_bg.png"
                            mode="fit"
                            c:s-server-addr-tb-img>
                            <textinput
                                s:font-size=text_size
                                value=server_addr
                                with=ServerAddressTextBox>
                            </textinput>
                        </img>
                        // Save button
                        <button
                            c:s-bottom-button
                            id="save-button"
                            s:width=format!("{}px", tile_size * 3.6)
                            s:height=format!("{}px", tile_size * 0.9)
                            s:left=format!("{}px", tile_size * 0.875)
                        >
                            <img src=button_handle.clone_weak() mode="fit">
                                <span s:font-size=text_size>"Save"</span>
                            </img>
                        </button>
                        // Successful save indicator text
                        <label
                            value="Settings Saved Successfully"
                            c:hidden
                            c:s-saved-indicator
                            id="successful-save"
                            s:font-size=text_size
                        ></label>
                        // Failed save indicator text
                        <label
                            value="Failed to Save the Settings!"
                            c:hidden
                            id="fail-save"
                            c:s-saved-indicator
                            s:font-size=text_size
                        ></label>
                    </div>
                    //
                    // Right column
                    //
                    <div c:s-right-column>
                        // Debug mode text
                        <label
                            value="Debug Mode"
                            s:font-size=text_size
                            c:s-top-text>
                        </label>
                        // Debug mode switch
                        <button
                            {switch}
                            with=switch_comp
                            c:s-debug-mode-switch
                            s:width=format!("{}px", tile_size * 1.6)
                            s:height=format!("{}px",tile_size * 0.8)
                            on:press=run!(|switch: &mut Switch|{
                                switch.on = !switch.on;
                                if switch.on{
                                    switch.texture = switch_handle_on.clone_weak();
                                }
                                else{
                                    switch.texture = switch_handle_off.clone_weak();
                                }})
                            >
                            <img {switch_img} src=switch_handle_on.clone_weak()>
                            </img>
                        </button>
                        // Back button
                        <button
                            c:s-bottom-button
                            id="back-button"
                            s:width=format!("{}px", tile_size * 3.6)
                            s:height=format!("{}px", tile_size * 0.9)
                            s:left=format!("{}px", tile_size * 0.875)>
                            <img src=button_handle mode="fit">
                                <span s:font-size=text_size>"Back"</span>
                            </img>
                        </button>
                    </div>
                </div>
            </img>
        </body>
    });
    commands.add(from!(switch, Switch: texture) >> to!(switch_img, Img: src));
}

fn settings_ui(
    mut settings: ResMut<Settings>,
    mut state: ResMut<State<GameState>>,
    mut elements: Elements,
    mut reader: EventReader<BtnEvent>,
    username_input_q: Query<&TextInput, (With<UsernameTextBox>, Without<ServerAddressTextBox>)>,
    server_addr_input_q: Query<&TextInput, (With<ServerAddressTextBox>, Without<UsernameTextBox>)>,
    switch_input_q: Query<&Switch, (Without<ServerAddressTextBox>, Without<UsernameTextBox>)>,
) {
    for event in reader.iter() {
        if let BtnEvent::Pressed(entity) = event {
            if let Some(save_btn_ent) = elements.select("#save-button").entities().get(0) {
                if save_btn_ent == entity {
                    settings.debug_mode = switch_input_q.single().on;
                    settings.username = username_input_q
                        .single()
                        .value
                        .trim()
                        .replace(|c: char| !c.is_ascii() || c.is_whitespace(), "");
                    settings.server_addr = server_addr_input_q
                        .single()
                        .value
                        .trim()
                        .replace(|c: char| !c.is_ascii() || c.is_whitespace(), "");
                    match write_settings_to_file(&settings) {
                        Ok(_) => {
                            elements.select("#successful-save").remove_class("hidden");
                            elements.select("#fail-save").add_class("hidden");
                        }
                        Err(_) => {
                            elements.select("#fail-save").remove_class("hidden");
                            elements.select("#successful-save").add_class("hidden");
                        }
                    }
                }
            }
            if let Some(back_btn_ent) = elements.select("#back-button").entities().get(0) {
                if back_btn_ent == entity {
                    state.set(GameState::Waiting).unwrap();
                }
            }
        }
    }
}
