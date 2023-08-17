use super::*;
use common::card::{CardEntity, CardNameToSprite};

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(placing_troop))
            .insert_resource(CurrentlyPlacing(false))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_in_game_ui))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(in_game_ui_left_panel))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_currency_text))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(chat_ui))
            .insert_resource(EditingMessage("".to_owned()));
    }
}

#[derive(Component)]
struct CurrentlyPlacingCard(Card);

#[derive(Resource)]
struct CurrentlyPlacing(bool);

#[derive(Resource)]
struct EditingMessage(String);

#[derive(Component, Default)]
struct ChatSendButton;
#[derive(Component, Default)]
struct ChatTextBox;
#[derive(Component, Default)]
pub struct TurnIndicator;
#[derive(Component, Default)]
struct SpiritIndicator;
#[derive(Component, Default)]
struct PawnIndicator;

#[derive(Clone, Resource, Debug)]
struct UiCardElement{
    card_button_ent: Entity,
    card: Card,
    name: String,
    troop_img: Handle<Image>,
}

#[derive(Resource)]
struct UiCardElementList(Vec<UiCardElement>);

fn spawn_in_game_ui(
    mut commands: Commands,
    game_font: Res<GameFont>,
    tile_size: Res<TileSize>,
    asset_server: Res<AssetServer>,
    _card_sprites: Res<CardSprites>,
    deck: Res<Deck>,
    card_name_to_sprite: Res<CardNameToSprite>,
    is_self_turn: Res<IsSelfTurn>,
) {
    let ui_card_bg_button: Handle<Image> = asset_server.load("ui_card_bg_button.png");
    let spirit_img_handle: Handle<Image> = asset_server.load("spirit.png");
    let pawn_img_handle: Handle<Image> = asset_server.load("pawn.png");
    let tile_size = tile_size.0;
    let text_size = tile_size / 4.5;
    let img_size = format!("{}px", tile_size / 2.0);
    let chat_send_height = format!("{}px", tile_size * 0.6875);
    let mut ui_card_button_elem_list: Vec<UiCardElement> = Vec::new();
    let turn_label_value = if is_self_turn.0{
        "Your Turn"
    } else{
        "Opponent's Turn"
    };

    for i in 0..5{

        let card = deck.0.get(4 - i).unwrap().clone();
        let card_button_ent = commands.spawn_empty().id();
        let temp = &card.get_name();
        let mut chars: Vec<char> = temp.chars().collect();
        chars[0] = chars[0].to_uppercase().nth(0).unwrap();
        let name: String = chars.into_iter().collect();
        let troop_img: Handle<Image> = asset_server
            .load(format!(
                "troop_{}.png",
                card_name_to_sprite
                    .0
                    .get(&deck.0.get(4 - i).unwrap().get_name())
                    .unwrap()
        ));
        ui_card_button_elem_list.push(UiCardElement { card_button_ent, name, troop_img, card: card.clone() });
    }

    commands.insert_resource(UiCardElementList(ui_card_button_elem_list.clone()));

    commands.add(
        eml!{
            <body>
                <div id="left-panel">
                    <label 
                        id="turn-label" 
                        value=turn_label_value 
                        with=TurnIndicator
                        s:font-size=format!("{}", tile_size / 3.0)>
                    </label>
                    <div id="currency-indicator-section">
                        <label 
                            value="0" 
                            with=SpiritIndicator 
                            c:indicator-label
                            s:font-size=format!("{}", tile_size / 2.5)>
                        </label>
                        <img 
                            src=spirit_img_handle
                            s:width=img_size.clone()
                            s:height=img_size.clone()
                            s:position-type="absolute"
                            s:left="10%"
                            mode="fit"
                        >
                        </img>
                        <label 
                            value="0" 
                            with=PawnIndicator
                            c:indicator-label
                            s:left="50%"
                            s:font-size=format!("{}", tile_size / 2.5)>
                        </label>
                        <img 
                            src=pawn_img_handle
                            s:width=img_size.clone()
                            s:height=img_size
                            s:position-type="absolute"
                            s:left="60%"
                            mode="fit"
                        >
                        </img>
                    </div>
                    <for i in = ui_card_button_elem_list.iter().zip(0..5)> 
                        <div 
                            s:height=format!("{}px", tile_size * 1.2) 
                            s:margin=format!("{}px", tile_size / 5.0)
                            s:bottom=format!("{}px", i.1 as f32 * tile_size * 1.2)
                            c:ig-cell
                        >
                            <div c:ig-cell-text-div>
                                <label 
                                    value=format!("{} [{} Spirits]", i.0.name, i.0.card.get_cost())
                                    s:font-size=text_size
                                    s:color="black">
                                </label>
                            </div>
                            
                            <div 
                                c:ig-cell-img-div
                                s:height=format!("{}px", tile_size)
                                s:width=format!("{}px", tile_size)
                            >
                                <button c:ig-cell-button {i.0.card_button_ent}>
                                    <img mode="fit" src=ui_card_bg_button.clone()>
                                        <img mode="fit" src=i.0.troop_img.clone() c:ig-cell-troop-img-div>
                                        </img>
                                    </img>
                                </button>
                            </div>
                        </div>
                    </for>
                </div>
                <div id="right-panel">
                    <div id="card-viewing-div">
                    </div>
                    <div id="chat-div">
                        <label 
                            value="Chat" 
                            s:font-size=format!("{}", tile_size / 4.0) 
                            s:color="black">
                        </label>
                        <div id="chat-area"> 
                            <for _ in = 0..7>
                                <label
                                    s:color="black"
                                    value=""
                                    s:font-size=format!("{}", tile_size / 4.0)
                                    s:top="0px"
                                    s:width="90%"
                                    s:left="0%"
                                    c:chat-message
                                    >
                                </label>
                            </for>
                        </div>
                        <img
                            src="text_box_bg.png"
                            mode="fit"
                            s:width=format!("{}px", tile_size * 3.18)
                            s:height=chat_send_height.clone()
                            s:position-type="absolute"
                            s:left=format!("{}px", tile_size * 0.466)
                            s:bottom="0%"
                        >
                            <textinput
                                with=ChatTextBox
                                s:font-size=format!("{}", tile_size / 4.5)
                                value="">
                            </textinput>
                        </img>
                        <button
                            id="chat-send-button"
                            s:height=chat_send_height.clone()
                            s:width=chat_send_height
                            s:position-type="absolute"
                            s:bottom="0%"
                            s:left=format!("{}px", tile_size * 3.846)
                            s:margin="0%"
                            with=ChatSendButton 
                        >
                            <img src="send_button.png" mode="fit"> 
                            </img>
                        </button>        
                    </div>
                </div>
            </body>
        }
    );
}

fn update_currency_text(
    mut spirit_text_query: Query<&mut Label, With<SpiritIndicator>>,
    mut pawn_text_query: Query<&mut Label, (Without<SpiritIndicator>, With<PawnIndicator>)>,
    spirits: Res<Spirits>,
    pawns: Res<Pawns>,
){
    spirit_text_query.single_mut().value = format!("{}", spirits.0);
    pawn_text_query.single_mut().value = format!("{}", pawns.0);
}

fn chat_ui(
    queue_out: ResMut<QueueOut>,
    mut text_box_query: Query<&mut TextInput, With<ChatTextBox>>,
    mut reader: EventReader<BtnEvent>,
    chat_send_button_query: Query<Entity, (With<ChatSendButton>, Without<ChatTextBox>)>,
){
    let chat_send_btn_ent = chat_send_button_query.single();
    for event in reader.iter(){
        if let BtnEvent::Pressed(entity) = event{
            if chat_send_btn_ent == *entity{
                let mut text_box = text_box_query.single_mut();
                queue_out
                    .0
                    .lock()
                    .unwrap()
                    .push_back(ClientMessage::ChatMessage(text_box.value.clone()));
                text_box.value = "".to_string();
            }
        }
    }
}

fn in_game_ui_left_panel(
    /*
    mut context: ResMut<EguiContext>,
    selected_card: Res<ViewingCardEntity>,
    */
    queue_out: ResMut<QueueOut>,
    mut is_self_turn: ResMut<IsSelfTurn>,
    deck: Res<Deck>,
    spirit_count: Res<Spirits>,
    pawn_count: Res<Pawns>,
    is_player_1: Res<IsPlayer1>,
    card_sprites: Res<CardSprites>,
    tile_size: Res<TileSize>,
    mut commands: Commands,
    mut is_placing: ResMut<CurrentlyPlacing>,
    mut card_entity_q: Query<&mut CardEntity, Without<Label>>,
    mut editing_message: ResMut<EditingMessage>,
    mut elements: Elements,
    mut reader: EventReader<BtnEvent>,
    ui_card_button_elements: Res<UiCardElementList>,
) {
    for event in reader.iter(){
        if let BtnEvent::Pressed(entity) = event{
            if let Some(button_ent) = elements.select("#end-turn-button").entities().get(0) {
                if button_ent == entity{
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
                    elements.select("#end-turn-button").remove();
                    commands.add(|world: &mut World|{
                        world.query_filtered::<&mut Label, With<TurnIndicator>>().single_mut(world).value = "Opponent's Turn".to_string();
                    });
                }
            
                else {
                    let temp = ui_card_button_elements
                        .0
                        .iter()
                        .filter(|x| x.card_button_ent == *entity)
                        .collect::<Vec<&UiCardElement>>();

                    if let Some(element) = temp.get(0){
                        let card = element.card.clone();
                        if !is_placing.0
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
                     
                }
            }
        } 
    }
    /*
    egui::SidePanel::left("in_game_ui")
        .min_width(tile_size.0 * 4.75)
        .max_width(tile_size.0 * 4.75)
        .show(context.ctx_mut(), |ui| {
            if is_self_turn.0 {
                ui.label("Your turn!");
                ui.label("Opponent's turn!");
            }
            ui.add_space(10.0);
            /*
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
                */
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
    */
}

fn placing_troop(
    queue_out: ResMut<QueueOut>,
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
