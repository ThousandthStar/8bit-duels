use bevy::prelude::*;

use crate::{ui::GameFont, GameState};
use std::time::Duration;

pub struct OpeningPlugin;

impl Plugin for OpeningPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Opening).with_system(spawn_opening))
            .add_system_set(SystemSet::on_update(GameState::Opening).with_system(opening))
            .add_system_set(SystemSet::on_exit(GameState::Opening).with_system(exit_opening))
            .insert_resource(OpeningTimer {
                timer: Timer::new(Duration::from_millis(1500), TimerMode::Once),
                going_up: true,
            });
    }
}

#[derive(Resource)]
struct OpeningTimer {
    timer: Timer,
    going_up: bool,
}

//marker component
#[derive(Component)]
struct ParentNode;

fn opening(
    mut timer: ResMut<OpeningTimer>,
    mut parent_node_q: Query<&mut Style, With<ParentNode>>,
    mut text_q: Query<&mut Text, Without<ParentNode>>,
    time: Res<Time>,
    mut state: ResMut<State<GameState>>,
) {
    timer.timer.tick(time.delta());
    if !timer.going_up {
        if timer.timer.finished() {
            state.set(GameState::Waiting);
        } else {
            for mut text in text_q.iter_mut() {
                let alpha = text.sections[0].style.color.a();
                text.sections[0].style.color =
                    Color::rgba(1.0, 1.0, 1.0, alpha - (time.delta_seconds() / 1.5));
            }
        }
    } else {
        if timer.timer.finished() {
            timer.timer = Timer::new(Duration::from_millis(3000), TimerMode::Once);
            timer.going_up = false;
        } else {
            for mut style in parent_node_q.iter_mut() {
                if let Val::Percent(top) = style.position.top {
                    style.position.top = Val::Percent(top - (5.0 * time.delta_seconds()));
                }
            }
            for mut text in text_q.iter_mut() {
                let alpha = text.sections[0].style.color.a();
                text.sections[0].style.color =
                    Color::rgba(1.0, 1.0, 1.0, alpha + (time.delta_seconds() / 1.5));
            }
        }
    }
}

fn exit_opening(node_query: Query<Entity, With<Node>>, mut commands: Commands) {
    for entity in node_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_opening(
    mut commands: Commands,
    windows: Res<Windows>,
    _asset_server: Res<AssetServer>,
    game_font: Res<GameFont>,
) {
    let window = windows.get_primary().unwrap();
    let height = window.height();
    commands
        .spawn(NodeBundle {
            background_color: Color::rgb(0.0, 0.0, 0.0).into(),
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
                        position: UiRect::new(
                            Val::Percent(30.0),
                            Val::Auto,
                            Val::Percent(30.0),
                            Val::Auto,
                        ),
                        size: Size::new(Val::Percent(40.0), Val::Percent(10.0)),
                        ..default()
                    },

                    ..default()
                })
                .insert(ParentNode)
                .with_children(|parent| {
                    let mut bundle = TextBundle::from_section(
                        "ThousandthStar Games",
                        TextStyle {
                            color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                            font: game_font.0.clone_weak(),
                            font_size: height / 20.0,
                        },
                    )
                    .with_text_alignment(TextAlignment::CENTER);
                    bundle.style = Style {
                        position: UiRect::new(
                            Val::Auto,
                            Val::Percent(30.0),
                            Val::Auto,
                            Val::Percent(0.0),
                        ),
                        align_self: AlignSelf::FlexEnd,
                        ..default()
                    };
                    parent.spawn(bundle);
                });
        });
}
