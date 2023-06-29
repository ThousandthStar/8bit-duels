use bevy::prelude::*;

pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_switch_textures)
            .add_system(switches)
            .add_system(text_box_system)
            .insert_resource(SelectedTextBox(None))
            .add_system(button_hover);
    }
}

//*************************** SWITCHES *************************************

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
pub struct Switch {
    // true: on
    // false: off
    pub on: bool,
}

fn switches(
    mut query: Query<
        (&Interaction, &mut UiImage, &mut Switch),
        (With<Button>, Changed<Interaction>),
    >,
    switch_textures: Res<SwitchTextures>,
) {
    for (interaction, mut image, mut switch) in query.iter_mut() {
        if matches!(*interaction, Interaction::Clicked) {
            switch.on = !switch.on;
        }
        if switch.on {
            image.0 = switch_textures.on.clone_weak();
        } else {
            image.0 = switch_textures.off.clone_weak();
        }
    }
}

//**************************************************************************
//*************************** TEXT BOXES ***********************************

#[derive(Component)]
pub struct TextBoxInput {
    pub max: usize,
}

/// marker component
#[derive(Component)]
pub struct TextBoxButton;

#[derive(Resource)]
pub struct SelectedTextBox(Option<Entity>);

fn text_box_system(
    mut text_box_query: Query<(&mut Text, &mut TextBoxInput), Without<Button>>,
    mut button_query: Query<(&Interaction, &Children), (With<Button>, With<TextBoxButton>)>,
    input: Res<Input<KeyCode>>,
    mut events: EventReader<ReceivedCharacter>,
    mut selected_text_box: ResMut<SelectedTextBox>,
) {
    let mut string_input = "".to_string();
    events.iter().for_each(|x| {
        if x.char.to_string() != " " && x.char.to_string() != "\n" && x.char.to_string() != "" {
            string_input += &x.char.to_string();
        }
    });
    for (interaction, children) in button_query.iter_mut() {
        if matches!(interaction, Interaction::Clicked) {
            selected_text_box.0 = Some(children[0].clone());
            break;
        }
    }
    if let Some(text_box_entity) = selected_text_box.0 {
        if let Ok((mut text, text_box_input)) = text_box_query.get_mut(text_box_entity) {
            let mut value = text.sections.get(0).unwrap().value.clone();
            if input.just_pressed(KeyCode::Back) {
                let length = value.len();
                if length >= 1 {
                    value.truncate(length - 1);
                }
            }
            else{
                value += &string_input;
            }
            value.truncate(text_box_input.max);
            text.sections.get_mut(0).unwrap().value = value;
        }
    }
}

//**************************************************************************

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
