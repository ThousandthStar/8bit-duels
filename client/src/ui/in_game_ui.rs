use super::*;
use common::card::{CardEntity, CardNameToSprite};
use widgets::{TextBoxInput, TextBoxButton};

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(placing_troop))
            .insert_resource(CurrentlyPlacing(false))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_in_game_ui))
            .insert_resource(ChatMessages(Vec::new()))
            .insert_resource(EditingMessage("".to_owned()));
    }
}

#[derive(Component)]
struct CurrentlyPlacingCard(Card);

#[derive(Resource)]
struct CurrentlyPlacing(bool);

#[derive(Resource)]
struct EditingMessage(String);

#[derive(Resource)]
pub struct ChatMessages(pub Vec<String>);

#[derive(Component)]
struct ChatSendButton;
#[derive(Component)]
pub struct ChatText;
#[derive(Component)]
struct ChatTextBox;

fn spawn_in_game_ui(
    mut commands: Commands,
    game_font: Res<GameFont>,
    tile_size: Res<TileSize>,
    asset_server: Res<AssetServer>,
    card_sprites: Res<CardSprites>,
    deck: Res<Deck>,
    card_name_to_sprite: Res<CardNameToSprite>,
    is_self_turn: Res<IsSelfTurn>,
    spirit_count: Res<Spirits>,
    pawn_count: Res<Pawns>,
){
    // left panel
    commands.spawn(NodeBundle{
        style: Style{
            size: Size { width: Val::Percent(33.0), height: Val::Percent(100.0) },
            position: UiRect {left: Val::Px(0.0), top: Val::Px(0.0), ..default()},
            ..default()
        },
        background_color: Color::WHITE.into(),
        ..default()
    }).with_children(|parent|{
            parent.spawn(TextBundle::from_section(if is_self_turn.0 { "Your turn" } else { "Opponent's turn" }, TextStyle{
                font: game_font.0.clone_weak(),
                font_size: tile_size.0 / 3.0,
                color: Color::BLACK.into(),
            }).with_style(Style{
                    margin: UiRect::all(Val::Percent(5.0)),
                    ..default()
                })); 
            if is_self_turn.0 {
                parent.spawn(ButtonBundle{
                    image: asset_server.load("button.png").into(),
                    style: Style{
                        size: Size::new(Val::Px(tile_size.0 * 3.6), Val::Px(tile_size.0 * 0.9)), 
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        position: UiRect {
                            top: Val::Percent(12.0),
                            left: Val::Percent(5.0),
                            ..default()
                        },
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    ..default()
                }).with_children(|parent|{
                        parent.spawn(TextBundle::from_section("End turn", TextStyle{
                                font: game_font.0.clone_weak(), 
                                font_size: tile_size.0 / 4.0,
                                color: Color::WHITE.into()
                            }).with_text_alignment(TextAlignment::CENTER));
                    });
            }

            
            for i in 0..5 {
                let card = deck.0.get(4 - i).unwrap().clone();
                parent.spawn(NodeBundle{
                    style: Style{
                        size: Size::new(Val::Percent(100.0), Val::Px(tile_size.0 * 1.2)),
                        position: UiRect { left: Val::Px(0.0), bottom: Val::Px(i as f32 * tile_size.0 * 1.2), ..default()},        
                        position_type: PositionType::Absolute,
                        margin: UiRect::bottom(Val::Px(tile_size.0 / 5.0)),
                        padding: UiRect::all(Val::Percent(5.0)),
                        ..default()
                    }, 
                    background_color: Color::WHITE.into(),
                    ..default()
                }).with_children(|parent|{
                        parent.spawn(NodeBundle{
                            style: Style{
                                size: Size::new(Val::Percent(66.6), Val::Percent(100.0)),
                                align_items: AlignItems::Center,
                                align_content: AlignContent::Center,
                                ..default()
                            },
                            ..default()
                        }).with_children(|parent|{
                                let temp = &card.get_name();
                                let mut chars: Vec<char> = temp.chars().collect();
                                chars[0] = chars[0].to_uppercase().nth(0).unwrap();
                                let name: String = chars.into_iter().collect();
                                parent.spawn(TextBundle::from_section(format!("{} [{} spirits]", name, card.get_cost()), TextStyle{
                                    font: game_font.0.clone_weak(),
                                    font_size: tile_size.0 / 4.5,
                                    color: Color::BLACK.into(),
                                }));
                            });
                        parent.spawn(ImageBundle{
                            style: Style{
                                position: UiRect { right: Val::Percent(5.0), ..default() },
                                position_type: PositionType::Absolute,
                                size: Size::new(Val::Px(tile_size.0), Val::Px(tile_size.0)),
                                ..default()
                            },
                            image: asset_server.load("ui_card_bg_button.png").into(),
                            ..default()
                        }).with_children(|parent|{
                                parent.spawn(ImageBundle{
                                    image: asset_server.load(format!("troop_{}.png", card_name_to_sprite.0.get(&deck.0.get(4 - i).unwrap().get_name()).unwrap())).into(),
                                    style: Style{
                                        size: Size::new(Val::Percent(85.0), Val::Percent(85.0)),
                                        position: UiRect{
                                            left: Val::Percent(7.5),
                                            top: Val::Percent(7.5),
                                            ..default()
                                        },
                                        position_type: PositionType::Absolute,
                                        ..default()
                                    },
                                    ..default()
                                });
                            });
                    });
            }
        });
    /*
    * Right panel
    */
    commands.spawn(NodeBundle{
        style: Style{
            size: Size::new(Val::Percent(33.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                right: Val::Px(0.0),
                ..default()
            },
            ..default()
        },
        background_color: Color::WHITE.into(),
        ..default()
    }).with_children(|parent|{
            parent.spawn(NodeBundle{
                style: Style{
                    position_type: PositionType::Absolute,
                    position: UiRect { top: Val::Px(0.0), ..Default::default() },
                    size: Size::new(Val::Percent(100.0), Val::Percent(85.0)),
                    ..Default::default()
                },
                ..Default::default()
            }).with_children(|parent|{
                    parent.spawn(TextBundle::from_section("Chat", TextStyle{
                        font: game_font.0.clone_weak(),
                        font_size: tile_size.0 / 3.0,
                        color: Color::BLACK.into(),
                    }).with_style(Style{
                            margin: UiRect{left: Val::Px(tile_size.0 * 0.466), top: Val::Percent(2.5), ..Default::default()},
                            ..Default::default()
                        }));
                    parent.spawn(NodeBundle{
                        style: Style{
                            size: Size::new(Val::Percent(100.0), Val::Percent(85.0)),
                            position_type: PositionType::Absolute,
                            position: UiRect{top: Val::Percent(10.0), ..Default::default()},
                            overflow: Overflow::Hidden,
                            ..Default::default()
                        },
                        ..Default::default()
                    }).with_children(|parent|{
                            parent.spawn(TextBundle::from_section("", TextStyle{
                                font: game_font.0.clone_weak(),
                                font_size: tile_size.0 / 4.0,
                                color: Color::BLACK.into(),
                            }).with_style(Style{
                                    position_type: PositionType::Absolute,
                                    position: UiRect {left: Val::Px(tile_size.0 * 0.466), bottom: Val::Px(0.0), ..Default::default()},
                                    ..Default::default()
                                })).insert(ChatText);
                        });
                });

            // Chat send area
            parent.spawn(NodeBundle{
                style: Style{
                    size: Size::new(Val::Percent(30.3), Val::Px(tile_size.0 * 0.6875)),
                    position_type: PositionType::Absolute,
                    position: UiRect{ bottom: Val::Percent(5.0), ..default() },
                    ..default()
                },
                ..default()
            }).with_children(|parent|{
                    parent.spawn(ButtonBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::FlexStart,
                                position_type: PositionType::Absolute,
                                position: UiRect { left: Val::Px(tile_size.0 * 0.466), ..Default::default() },
                                size: Size::new(Val::Px(tile_size.0 * 3.18), Val::Percent(100.0)),
                                ..default()
                            },
                            image: asset_server.load("text_box_bg.png").into(),
                            ..default()
                        })
                        .insert(TextBoxButton)
                        .with_children(|parent| {
                            parent
                                .spawn(
                                    TextBundle::from_section(
                                        "",
                                        TextStyle {
                                            font: game_font.0.clone_weak(),
                                            font_size: tile_size.0 / 4.5,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_style(Style {
                                        margin: UiRect {
                                            left: Val::Percent(5.0),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    }),
                                )
                                .insert(TextBoxInput { max: 20 })
                                .insert(ChatTextBox);
                        });
                    parent.spawn(ButtonBundle{
                        image: asset_server.load("send_button.png").into(),
                        style: Style{
                            size: Size::new(Val::Px(tile_size.0 * 0.6875), Val::Px(tile_size.0 * 0.6875)),
                            position_type: PositionType::Absolute,
                            position: UiRect {left: Val::Px(tile_size.0 * 3.846), ..Default::default()},
                            ..default()
                        },
                        ..default()
                    }).insert(ChatSendButton);
                });
        });
}

fn in_game_ui(
    mut context: ResMut<EguiContext>,
    selected_card: Res<ViewingCardEntity>,
    mut queue_out: ResMut<QueueOut>,
    mut is_self_turn: ResMut<IsSelfTurn>,
    deck: Res<Deck>,
    spirit_count: Res<Spirits>,
    pawn_count: Res<Pawns>,
    is_player_1: Res<IsPlayer1>,
    card_sprites: Res<CardSprites>,
    tile_size: Res<TileSize>,
    mut commands: Commands,
    mut is_placing: ResMut<CurrentlyPlacing>,
    mut card_entity_q: Query<&mut CardEntity>,
    chat_messages: Res<ChatMessages>,
    mut editing_message: ResMut<EditingMessage>,
) {
    egui::SidePanel::left("in_game_ui")
        .min_width(tile_size.0 * 4.75)
        .max_width(tile_size.0 * 4.75)
        .show(context.ctx_mut(), |ui| {
            if is_self_turn.0 {
                ui.label("Your turn!");
                if ui.button("End turn").clicked() {
                    queue_out
                        .0
                        .lock()
                        .unwrap()
                        .push_back(ClientMessage::EndTurn);
                    is_self_turn.0 = false;
                    for mut card_entity in card_entity_q.iter_mut() {
                        if card_entity.is_owned_by_p1() != is_player_1.0 {
                            card_entity.reset();
                        }
                    }
                }
            } else {
                ui.label("Opponent's turn!");
            }
            ui.add_space(10.0);
            if let Some(card_entity) = selected_card.0.clone() {
                if card_entity.is_owned_by_p1() == is_player_1.0 {
                    ui.monospace("Your troop");
                } else {
                    ui.monospace("Your opponent's troop");
                }
                ui.monospace(format!("Current card: {}", card_entity.get_card().name));
                ui.monospace(format!("Attack: {}", card_entity.get_card().get_damage()));
                ui.monospace(format!("HP: {}", card_entity.current_hp));
                if card_entity.stun_count > 0 {
                    ui.monospace("Stunned");
                }
                if card_entity.get_y_pos() == 0 {
                    if !card_entity.has_moved()
                        && card_entity.is_owned_by_p1() == is_player_1.0
                        && is_self_turn.0
                        && !(card_entity.stun_count > 0)
                    {
                        if ui.button("Win").clicked() {
                            let y = if is_player_1.0 {
                                card_entity.get_y_pos()
                            } else {
                                8 - card_entity.get_y_pos()
                            };
                            let x = if is_player_1.0 {
                                card_entity.get_x_pos()
                            } else {
                                4 - card_entity.get_x_pos()
                            };
                            queue_out
                                .0
                                .lock()
                                .unwrap()
                                .push_back(ClientMessage::WinGame(x, y));
                        }
                    }
                }
            }
            ui.add_space(10.0);
            ui.monospace(format!("Pawns: {}", pawn_count.0));
            ui.monospace(format!("Spirits: {}", spirit_count.0));
            ui.add_space(10.0);
            ui.monospace("Spawn card");
            for card in &deck.0 {
                if ui.button(card.get_name()).clicked()
                    && !is_placing.0
                    && pawn_count.0 > 0
                    && spirit_count.0 >= card.get_cost()
                    && is_self_turn.0
                {
                    is_placing.0 = true;
                    let mut sprite = TextureAtlasSprite::new(
                        card_sprites.1.get(&card.get_name()).unwrap().clone(),
                    );
                    sprite.custom_size = Some(Vec2::splat(tile_size.0 * 0.8));

                    commands
                        .spawn(SpriteSheetBundle {
                            sprite,
                            texture_atlas: card_sprites.0.clone(),
                            transform: Transform::from_xyz(100000000.0, 10000000.0, 999.0),
                            ..Default::default()
                        })
                        .insert(CurrentlyPlacingCard(card.clone()));
                }
            }
            ui.add_space(30.0);
        });

    egui::SidePanel::right("in_game_ui_right")
        .min_width(tile_size.0 * 5.0)
        .max_width(tile_size.0 * 5.0)
        .show(context.ctx_mut(), |ui| {
            ui.label("Chat");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut editing_message.0);
                if ui.button("Send").clicked() {
                    queue_out
                        .0
                        .lock()
                        .unwrap()
                        .push_back(ClientMessage::ChatMessage(editing_message.0.clone()));
                    editing_message.0 = "".to_owned();
                }
            });

            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.set_min_width(f32::INFINITY);
                    for message in &chat_messages.0 {
                        ui.label(message);
                    }
                });
        });
}

fn placing_troop(
    mut queue_out: ResMut<QueueOut>,
    mut placing_query: Query<
        (Entity, Option<&CurrentlyPlacingCard>, &mut Transform),
        Without<Camera>,
    >,
    windows: Res<Windows>,
    cam_query: Query<(&Camera, &GlobalTransform)>,
    mouse: Res<Input<MouseButton>>,
    tile_size: Res<TileSize>,
    mut commands: Commands,
    mut is_placing: ResMut<CurrentlyPlacing>,
    mut pawn_count: ResMut<Pawns>,
    mut spirit_count: ResMut<Spirits>,
    is_player_1: Res<IsPlayer1>,
) {
    let (camera, global_transform) = cam_query.single();
    let window = windows.get_primary().unwrap();

    if let Some(pos) = window.cursor_position() {
        for (entity, option_placing_card, mut transform) in placing_query.iter_mut() {
            if let Some(currently_placing_card) = option_placing_card {
                let world_pos =
                    utils::screen_to_world_position(pos, &camera, &global_transform, &window);
                transform.translation.x = world_pos.x;
                transform.translation.y = world_pos.y;

                if mouse.just_pressed(MouseButton::Right) {
                    commands.entity(entity).despawn();
                    is_placing.0 = false;
                }

                if mouse.just_pressed(MouseButton::Left) {
                    let mut x = pos.x;
                    let mut y = pos.y;
                    if x < tile_size.0 * 5.0 || x > tile_size.0 * 10.0 {
                        return;
                    }
                    x -= tile_size.0 * 5.0;
                    x -= x % tile_size.0;
                    y -= y % tile_size.0;
                    x /= tile_size.0;
                    y /= tile_size.0;
                    if y > 3.0 {
                        return;
                    }
                    if is_player_1.0 {
                        y = 8.0 - y;
                    } else {
                        x = 4.0 - x;
                    }
                    queue_out
                        .0
                        .lock()
                        .unwrap()
                        .push_back(ClientMessage::SpawnCard(
                            currently_placing_card.0.clone(),
                            x as i32,
                            y as i32,
                        ));
                    commands.entity(entity).despawn();
                    is_placing.0 = false;
                    pawn_count.0 -= 1;
                    spirit_count.0 -= currently_placing_card.0.get_cost();
                    return;
                }
            }
        }
    }
}
