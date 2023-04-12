use bevy::prelude::*;

pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_switch_textures)
            .add_system(button_hover);
    }
}

#[derive(Resource)]
pub struct SwitchTextures {
    pub on: Handle<Image>,
    pub off: Handle<Image>,
}

fn load_switch_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SwitchTextures {
        on: asset_server.load("switch_on.png"),
        off: asset_server.load("switch_off.png"),
    });
}

fn button_hover(mut query: Query<(&Interaction, &mut BackgroundColor), With<Button>>) {
    for (interaction, mut color) in query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                color.0.set_a(0.9);
            }
            Interaction::None => {
                color.0.set_a(1.0);
            }
            _ => {}
        }
    }
}
