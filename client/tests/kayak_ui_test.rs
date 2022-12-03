use std::fmt::Display;

use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

#[derive(Copy, Clone, PartialEq)]
enum DropDownChoice {
    ChoiceOne,
    ChoiceTwo,
    ChoiceThree,
}

#[derive(Component, Clone, PartialEq, Default)]
struct ScrollBarButtonWidget;

impl Widget for ScrollBarButtonWidget {}

#[derive(Component, Clone, PartialEq)]
struct ScrollBarButtonState {
    show: bool,
    choice: Option<DropDownChoice>,
}

impl Default for ScrollBarButtonState {
    fn default() -> Self {
        Self {
            show: false,
            choice: None,
        }
    }
}

#[derive(Bundle)]
struct ScrollBarButtonBundle {
    pub styles: KStyle,
    pub children: KChildren,
    pub widget: ScrollBarButtonWidget,
}

fn scroll_bar_button_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    query: Query<&ScrollBarButtonState>
) -> bool {
    let state_entity = widget_context.use_static(&mut commands, entity, ScrollBarButtonState::default());
    if let Ok(state) = query.get(state_entity) {
        let parent_id = Some(entity);
        rsx! {
            <ElementBundle>
                <KButtonBundle
                    button= {KButton {
                        text: "test".into()
                    }}
                    styles = {KStyles::default()}
                    on_event = {OnEvent::new(
                        move |In((event_dispatcher_context, _, mut event, _entity)): In<(EventDispatcherContext, WidgetState, Event, Entity)>, 
                            mut query: Query<&ScrollBarButtonState>|{
                            event.prevent_default();
                            event.stop_propagation();
                            match event.event_type {
                                EvenType::Click(..) => {
                                    if let Ok(mut state) = query.get_mut(state_entity){
                                        state.show = true;
                                    }
                                }
                                _ => {}
                            }
                            (event_dispatcher_context, event)
                        }
                    }
                />
            </ElementBundle>
        }
    }
}

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("Monocraft.kayak_font"));

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <TextWidgetBundle
                text={TextProps {
                    content: "Hello World".into(),
                    size: 20.0,
                    ..Default::default()
                }}
            />
        </KayakAppBundle>
    }

    commands.spawn(UICameraBundle::new(widget_context));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
        .add_startup_system(startup)
        .run()
}
