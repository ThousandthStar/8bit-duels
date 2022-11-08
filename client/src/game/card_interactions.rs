use crate::{
    net::{packets, QueueOut},
    tilemap::TileSize,
    GameState, IsPlayer1, IsSelfTurn,
};
use bevy::prelude::*;

use super::card::CardEntity;

pub(crate) struct CardInteractions;

impl Plugin for CardInteractions {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedCardEntity(None))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(card_selecting_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(setting_indicators_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(card_interactions_system),
            );
    }
}

// components to flag movement and attack indicators
#[derive(Component)]
pub(crate) struct MoveIndicator(pub(crate) i32, pub(crate) i32);
#[derive(Component)]
pub(crate) struct AttackIndicator(pub(crate) i32, pub(crate) i32, pub(crate) f32);

pub(crate) struct SelectedCardEntity(pub(crate) Option<CardEntity>);

fn card_selecting_system(
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    tile_size: Res<TileSize>,
    cam_q: Query<(&Camera, &GlobalTransform), Without<CardEntity>>,
    is_player_1: Res<IsPlayer1>,
    mut card_entity_q: Query<(&Transform, &mut CardEntity), Without<Camera>>,
    mut selected_card_entity: ResMut<SelectedCardEntity>,
) {
    for (transform, mut card_entity) in card_entity_q.iter_mut() {
        if is_transform_clicked(transform, &mouse, &windows, &tile_size, &cam_q)
            && is_player_1.0 == card_entity.is_owned_by_p1()
        {
            selected_card_entity.0 = Some(card_entity.clone());
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

        for (move_indicator, mut visibility) in move_indicator_q.iter_mut() {
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
            if Vec2::new(attack_indicator.0 as f32, attack_indicator.1 as f32).distance(Vec2::new(
                selected_card_entity.get_x_pos() as f32,
                selected_card_entity.get_y_pos() as f32,
            )) < 15.
            {
                let mut available = false;
                for card_entity in card_entity_q.iter() {
                    if card_entity.get_x_pos() == attack_indicator.0
                        && card_entity.get_y_pos() == attack_indicator.1
                        && card_entity.is_owned_by_p1() != is_player_1.0
                    {
                        available = true;
                        attack_indicator.2 = selected_card_entity.card.get_damage();
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
    } else {
        for (move_indicator, mut visibility) in move_indicator_q.iter_mut() {
            visibility.is_visible = false;
        }
        for (attack_indicator, mut visibility) in attack_indicator_q.iter_mut() {
            visibility.is_visible = false;
        }
    }
}

fn card_interactions_system(
    mut queue_out: ResMut<QueueOut>,
    mut selected_card_entity: ResMut<SelectedCardEntity>,
    move_indicator_q: Query<
        (&MoveIndicator, &Visibility, &Transform),
        (Without<AttackIndicator>, Without<CardEntity>),
    >,
    attack_indicator_q: Query<
        (&AttackIndicator, &Visibility, &Transform),
        (Without<MoveIndicator>, Without<CardEntity>),
    >,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    tile_size: Res<TileSize>,
    cam_q: Query<(&Camera, &GlobalTransform), Without<CardEntity>>,
    is_self_turn: Res<IsSelfTurn>,
) {
    if selected_card_entity.0.is_none() {
        return;
    }

    for (move_indicator, visibility, transform) in move_indicator_q.iter() {
        if is_transform_clicked(transform, &mouse, &windows, &tile_size, &cam_q) {
            if visibility.is_visible && is_self_turn.0 {
                let mut queue_out_guard = queue_out.0.lock().unwrap();
                queue_out_guard.push_back(packets::move_packet(
                    selected_card_entity.0.clone().unwrap().get_x_pos(),
                    selected_card_entity.0.clone().unwrap().get_y_pos(),
                    move_indicator.0,
                    move_indicator.1,
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
    cam_q: &Query<(&Camera, &GlobalTransform), Without<CardEntity>>,
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

pub(crate) fn screen_to_world_position(
    screen_pos: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    window: &Window,
) -> Vec2 {
    //***********************************************************************/
    //Found on the unofficial Bevy cheat book (https://bevy-cheatbook.github.io/cookbook/cursor2world.html)
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);

    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

    // reduce it to a 2D value
    let world_pos: Vec2 = world_pos.truncate();
    //***********************************************************************/
    return world_pos;
}
