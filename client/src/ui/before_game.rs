use super::*;


pub struct BeforeGamePlugin;

impl Plugin for BeforeGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::PreparingForGame).with_system(build_ui))
            .add_system_set(
                SystemSet::on_enter(GameState::PreparingForGame).with_system(send_deck_packet),
            )
            .add_system_set(
                SystemSet::on_update(GameState::PreparingForGame).with_system(update_waiting_text),
            )
            .add_system_set(SystemSet::on_enter(GameState::PreparingForGame).with_system(spawn_ui))
            .add_system_set(
                SystemSet::on_exit(GameState::PreparingForGame).with_system(remove_bg_image),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::PreparingForGame).with_system(destroy_ui),
            );
    }
}

#[derive(Component)]
struct WaitingText {
    timer: Timer,
    state: u8,
}

//marker component
#[derive(Component)]
struct BgImage;

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tile_size: Res<TileSize>,
    font: Res<GameFont>,
) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("ui_bg.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(tile_size.0 * 15.0, tile_size.0 * 9.0)),
                ..default()
            },
            ..default()
        })
        .insert(BgImage);
    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                position: UiRect {
                    left: Val::Percent(30.0),
                    top: Val::Percent(30.0),
                    bottom: Val::Percent(30.0),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Percent(40.0),
                    height: Val::Percent(40.0),
                },
                ..default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(
                    TextBundle::from_section(
                        "Waiting for Opponent...",
                        TextStyle {
                            color: Color::BLACK,
                            font: font.0.clone_weak(),
                            font_size: tile_size.0 / 3.0,
                        },
                    )
                    .with_style(Style {
                        position: UiRect {
                            top: Val::Percent(30.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                )
                .insert(WaitingText {
                    timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                    state: 0,
                });
        });
}

fn update_waiting_text(mut query: Query<(&mut WaitingText, &mut Text)>, time: Res<Time>) {
    let (mut waiting_text, mut text) = query.single_mut();
    waiting_text.timer.tick(time.delta());
    if waiting_text.timer.finished() {
        waiting_text.state += 1;
        let mut new_text = "Waiting for Opponent".to_owned();
        for _ in 0..waiting_text.state {
            new_text += ".";
        }
        text.sections[0].value = new_text;
        if waiting_text.state > 2 {
            waiting_text.state = 0;
        }
    }
}

fn send_deck_packet(
    settings: Res<Settings>,
    queue_out: ResMut<QueueOut>,
    mut commands: Commands,
) {
    queue_out
        .0
        .lock()
        .unwrap()
        .push_back(ClientMessage::PlayerInfo(
            settings.username.clone(),
            settings.deck.clone(),
        ));
    commands.insert_resource(Deck(settings.deck.clone()));
}

fn remove_bg_image(mut commands: Commands, query: Query<Entity, With<BgImage>>) {
    commands.entity(query.single()).despawn();
}
