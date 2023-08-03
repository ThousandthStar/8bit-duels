use crate::utils::screen_to_world_position;
use crate::{net::QueueOut, tilemap::TileSize, GameState, IsPlayer1, IsSelfTurn, MainCamera};
use bevy::prelude::*;

use common::{card::CardEntity, messages::ClientMessage};

pub(crate) struct CardInteractions;

impl Plugin for CardInteractions {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedCardEntity(None))
            .insert_resource(ViewingCardEntity(None))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(card_selecting_system.after(card_interactions_system)),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(setting_indicators_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(card_interactions_system),
            )
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_select_indicator);
    }
}

#[derive(Component)]
pub struct SelectIndicator;

fn spawn_select_indicator(
    mut commands: Commands,
    tile_size: Res<TileSize>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(10000000000.0, 0.0, 450.0),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(tile_size.0)),
                ..Default::default()
            },
            texture: asset_server.load("select_indicator.png"),
            ..Default::default()
        })
        .insert(SelectIndicator);
}

// components to flag movement and attack indicators
#[derive(Component)]
pub struct MoveIndicator(pub i32, pub i32);
#[derive(Component)]
pub struct AttackIndicator(pub i32, pub i32, pub f32);
#[derive(Resource, Clone, Debug)]
pub struct SelectedCardEntity(pub Option<CardEntity>);
#[derive(Resource, Clone, Debug)]
pub struct ViewingCardEntity(pub Option<CardEntity>);

fn card_selecting_system(
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    tile_size: Res<TileSize>,
    cam_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    is_player_1: Res<IsPlayer1>,
    mut card_entity_q: Query<(&Transform, &mut CardEntity), Without<MainCamera>>,
    mut selected_card_entity: ResMut<SelectedCardEntity>,
    mut viewing_card_entity: ResMut<ViewingCardEntity>,
    mut select_indicator_q: Query<
        (&mut Transform, &mut Visibility),
        (
            With<SelectIndicator>,
            Without<MainCamera>,
            Without<CardEntity>,
        ),
    >,
) {
    for (transform, card_entity) in card_entity_q.iter_mut() {
        if is_transform_clicked(transform, &mouse, &windows, &tile_size, &cam_q) {
            match selected_card_entity.0.clone() {
                Some(selected_card) => {
                    if selected_card.get_x_pos() == card_entity.get_x_pos()
                        && selected_card.get_y_pos() == card_entity.get_y_pos()
                    {
                        if card_entity.is_owned_by_p1() == is_player_1.0 {
                            selected_card_entity.0 = None;
                        }
                        viewing_card_entity.0 = None;
                    } else {
                        if card_entity.is_owned_by_p1() == is_player_1.0 {
                            selected_card_entity.0 = Some(card_entity.clone());
                        }
                        viewing_card_entity.0 = Some(card_entity.clone());
                    }
                }
                None => {
                    if let Some(viewing_card) = viewing_card_entity.0.clone() {
                        if viewing_card.get_x_pos() == card_entity.get_x_pos()
                            && viewing_card.get_y_pos() == card_entity.get_y_pos()
                        {
                            viewing_card_entity.0 = None;
                            return;
                        }
                    }
                    if card_entity.is_owned_by_p1() == is_player_1.0 {
                        selected_card_entity.0 = Some(card_entity.clone());
                    }
                    viewing_card_entity.0 = Some(card_entity.clone());
                }
            }
        }
    }
    if let Some(selected_card_entity) = &viewing_card_entity.0 {
        let (mut transform, mut visibility) = select_indicator_q.single_mut();
        visibility.is_visible = false;
        for (card_transform, card_entity) in card_entity_q.iter() {
            if card_entity == selected_card_entity {
                transform.translation.x = card_transform.translation.x;
                transform.translation.y = card_transform.translation.y;
                visibility.is_visible = true;
            }
        }
    }
}

fn setting_indicators_system(
    mut move_indicator_q: Query<
        (&MoveIndicator, &mut Visibility),
        (Without<AttackIndicator>, Without<CardEntity>),
    >,
    mut attack_indicator_q: Query<
        (&mut AttackIndicator, &mut Visibility),
        (Without<MoveIndicator>, Without<CardEntity>),
    >,
    card_entity_q: Query<&CardEntity, (Without<MoveIndicator>, Without<AttackIndicator>)>,
    selected_card_entity: Res<SelectedCardEntity>,
    is_player_1: Res<IsPlayer1>,
    is_self_turn: Res<IsSelfTurn>,
) {
    if selected_card_entity.0.is_some() {
        let selected_card_entity = selected_card_entity.0.clone().unwrap();
        if selected_card_entity.stun_count > 0 {
            return;
        }

        for (move_indicator, mut visibility) in move_indicator_q.iter_mut() {
            if selected_card_entity.has_moved() || selected_card_entity.has_attacked() {
                break;
            }
            if Vec2::new(move_indicator.0 as f32, move_indicator.1 as f32).distance(Vec2::new(
                selected_card_entity.get_x_pos() as f32,
                selected_card_entity.get_y_pos() as f32,
            )) < 1.5
            {
                let mut available = true;
                for card_entity in card_entity_q.iter() {
                    if card_entity.get_x_pos() == move_indicator.0
                        && card_entity.get_y_pos() == move_indicator.1
                    {
                        available = false;
                        break;
                    }
                }
                if available && is_self_turn.0 {
                    visibility.is_visible = true;
                }
            } else {
                visibility.is_visible = false;
            }
        }

        for (mut attack_indicator, mut visibility) in attack_indicator_q.iter_mut() {
            if selected_card_entity.has_attacked() {
                break;
            }
            if Vec2::new(attack_indicator.0 as f32, attack_indicator.1 as f32).distance(Vec2::new(
                selected_card_entity.get_x_pos() as f32,
                selected_card_entity.get_y_pos() as f32,
            )) < 1.5
            {
                let mut available = false;
                for card_entity in card_entity_q.iter() {
                    if card_entity.get_x_pos() == attack_indicator.0
                        && card_entity.get_y_pos() == attack_indicator.1
                        && card_entity.is_owned_by_p1() != is_player_1.0
                    {
                        available = true;
                        attack_indicator.2 = selected_card_entity.get_card().get_damage();
                        break;
                    }
                }
                if available && is_self_turn.0 && !selected_card_entity.has_attacked() {
                    visibility.is_visible = true;
                }
            } else {
                visibility.is_visible = false;
            }
        }
    } else {
        for (_, mut visibility) in move_indicator_q.iter_mut() {
            visibility.is_visible = false;
        }
        for (_, mut visibility) in attack_indicator_q.iter_mut() {
            visibility.is_visible = false;
        }
    }
}

fn card_interactions_system(
    queue_out: ResMut<QueueOut>,
    mut selected_card_entity: ResMut<SelectedCardEntity>,
    move_indicator_q: Query<
        (&MoveIndicator, &Visibility, &Transform),
        (
            Without<AttackIndicator>,
            Without<CardEntity>,
            Without<MainCamera>,
        ),
    >,
    attack_indicator_q: Query<
        (&AttackIndicator, &Visibility, &Transform),
        (
            Without<MoveIndicator>,
            Without<CardEntity>,
            Without<MainCamera>,
        ),
    >,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    tile_size: Res<TileSize>,
    cam_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut card_entity_q: Query<&mut CardEntity, (Without<MoveIndicator>, Without<AttackIndicator>)>,
) {
    if selected_card_entity.0.is_none() {
        return;
    }

    for (move_indicator, visibility, transform) in move_indicator_q.iter() {
        if is_transform_clicked(transform, &mouse, &windows, &tile_size, &cam_q) {
            if visibility.is_visible {
                let mut queue_out_guard = queue_out.0.lock().unwrap();
                let acting_entity = selected_card_entity.0.clone().unwrap();
                for mut card_entity in card_entity_q.iter_mut() {
                    if card_entity.get_x_pos() == acting_entity.get_x_pos()
                        && card_entity.get_y_pos() == acting_entity.get_y_pos()
                    {
                        card_entity.moved();
                    }
                }
                queue_out_guard.push_back(ClientMessage::MoveTroop(
                    acting_entity.get_x_pos(),
                    acting_entity.get_y_pos(),
                    move_indicator.0,
                    move_indicator.1,
                ));
                selected_card_entity.0 = None;
                return;
            }
        }
    }

    for (attack_indicator, visibility, transform) in attack_indicator_q.iter() {
        if is_transform_clicked(transform, &mouse, &windows, &tile_size, &cam_q) {
            if visibility.is_visible {
                let mut queue_out_guard = queue_out.0.lock().unwrap();
                let acting_entity = selected_card_entity.0.clone().unwrap();
                for mut card_entity in card_entity_q.iter_mut() {
                    if card_entity.get_x_pos() == acting_entity.get_x_pos()
                        && card_entity.get_y_pos() == acting_entity.get_y_pos()
                    {
                        card_entity.moved();
                    }
                }
                queue_out_guard.push_back(ClientMessage::AttackTroop(
                    acting_entity.get_x_pos(),
                    acting_entity.get_y_pos(),
                    attack_indicator.0,
                    attack_indicator.1,
                ));
                selected_card_entity.0 = None;
                return;
            }
        }
    }
}

pub(crate) fn is_transform_clicked(
    transform: &Transform,
    mouse: &Res<Input<MouseButton>>,
    windows: &Res<Windows>,
    tile_size: &Res<TileSize>,
    cam_q: &Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> bool {
    let window = windows.primary();
    if let Some(screen_pos) = window.cursor_position() {
        let (camera, camera_transform) = cam_q.single();
        let world_pos = screen_to_world_position(screen_pos, camera, camera_transform, window);

        if mouse.just_pressed(MouseButton::Left) {
            if is_in_boundary(
                Vec2::new(
                    transform.translation.x - tile_size.0 / 2.,
                    transform.translation.y + tile_size.0 / 2.,
                ),
                Vec2::new(
                    transform.translation.x + tile_size.0 / 2.,
                    transform.translation.y - tile_size.0 / 2.,
                ),
                world_pos,
            ) {
                return true;
            } else {
                return false;
            }
        }
    }
    return false;
}

pub(crate) fn is_in_boundary(bound_1: Vec2, bound_2: Vec2, position: Vec2) -> bool {
    if position.x > bound_1.x
        && position.x < bound_2.x
        && position.y < bound_1.y
        && position.y > bound_2.y
    {
        return true;
    }
    return false;
}
