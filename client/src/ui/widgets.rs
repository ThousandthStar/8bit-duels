use bevy::prelude::*;

pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_switch_textures)
            .add_system(switches)
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

#[derive(Component, Default)]
pub struct Switch{
    // true: on
    // false: off
    pub on: bool
}

fn switches(
    mut query: Query<(&Interaction, &mut UiImage, &mut Switch), (With<Button>, Changed<Interaction>)>,
    switch_textures: Res<SwitchTextures>,
){
    for (interaction, mut image, mut switch) in query.iter_mut(){
        if matches!(*interaction, Interaction::Clicked){
            switch.on = !switch.on;
        }
        if switch.on {
                image.0 = switch_textures.on.clone_weak();
            }
            else{
                image.0 = switch_textures.off.clone_weak();
            }
    }
} 

fn button_hover(
    mut query: Query<(&Interaction, &mut BackgroundColor), (With<Button>, Changed<Interaction>)>,
) {
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
