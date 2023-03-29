use crate::{
    animations::{AttackAnimation, MovementAnimation},
    tilemap::TileSize,
};
use bevy::prelude::*;
use common::card::CardEntity;

pub struct OwnershipIndicatorPlugin;

impl Plugin for OwnershipIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update);
    }
}

// marker component
#[derive(Component)]
pub struct OwnershipIndicator;

fn update(
    mut card_entity_q: Query<
        (
            &Children,
            Option<&MovementAnimation>,
            Option<&AttackAnimation>,
        ),
        With<CardEntity>,
    >,
    mut ownership_indicators: Query<
        (Entity, &mut Visibility),
        (Without<CardEntity>, With<OwnershipIndicator>),
    >,
) {
    for (entity, mut visibility) in ownership_indicators.iter_mut() {
        for (children, move_opt, atck_opt) in card_entity_q.iter() {
            if children.contains(&entity) {
                if move_opt.is_some() || atck_opt.is_some() {
                    visibility.is_visible = false;
                } else {
                    visibility.is_visible = true;
                }
            }
        }
    }
}
