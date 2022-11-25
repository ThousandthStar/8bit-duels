use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use std::fmt;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_system(ui_example)
        .insert_resource(ComboBoxResultContainer {
            result_1: Option::_1,
        })
        .run();
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Option {
    _1,
    _2,
    _3,
}

struct ComboBoxResultContainer {
    result_1: Option,
}

impl fmt::Display for Option {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

fn ui_example(
    mut egui_context: ResMut<EguiContext>,
    mut container: ResMut<ComboBoxResultContainer>,
) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        egui::ComboBox::from_label("")
            .selected_text(container.result_1.to_string())
            .show_ui(ui, |ui| {
                for option_ in [Option::_1, Option::_2, Option::_3] {
                    ui.selectable_value(&mut container.result_1, option_, option_.to_string());
                }
            });

        if ui.button("Test").clicked() {
            println!("{}", container.result_1);
        }
    });
}
