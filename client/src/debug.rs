use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics}};
use bevy_egui::{egui, EguiContext};


use crate::DevMode;

pub struct DebugPlugin;

impl Plugin for DebugPlugin{
    fn build(&self, app: &mut App) {
        app.add_system(debug_ui)
            .add_plugin(FrameTimeDiagnosticsPlugin::default());
    }
}

fn debug_ui(
    mut context: ResMut<EguiContext>,
    dev_mode: Res<DevMode>,
    diagnostics: Res<Diagnostics>
){
    if dev_mode.0{
        egui::Window::new("Debug").show(context.ctx_mut(), |ui|{
            // source : https://github.com/jomala/bevy_screen_diags/blob/master/src/lib.rs
            let fps = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).and_then(|fps| fps.average());
            if let Some(fps) = fps {
                ui.monospace(format!("FPS: {}", fps.round()));
            }
        });
    }
}

