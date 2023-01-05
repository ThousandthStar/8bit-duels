use crate::tilemap::TileSize;
use crate::GameState;
use bevy::prelude::*;
use common::card::CardEntity;

pub struct StunIndicatorPlugin;

impl Plugin for StunIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_stun_indicators)
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(show_stun_indicator),
            );
    }
}

#[derive(Component)]
pub struct StunIndicator;

fn spawn_stun_indicators(
    mut commands: Commands,
    tile_size: Res<TileSize>,
    asset_server: Res<AssetServer>,
) {
    for _ in 0..12 {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 200.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(tile_size.0)),
                    ..Default::default()
                },
                visibility: Visibility { is_visible: false },
                texture: asset_server.load("stun_indicator.png"),
                ..Default::default()
            })
            .insert(StunIndicator);
    }
}

fn show_stun_indicator(
    mut stun_indicator_q: Query<(&mut Transform, &mut Visibility), With<StunIndicator>>,
    mut card_entity_q: Query<(&CardEntity, &Transform), Without<StunIndicator>>,
) {
    let mut stun_indicator_list: Vec<(Mut<Transform>, Mut<Visibility>)> =
        stun_indicator_q.iter_mut().collect();
    card_entity_q.for_each_mut(|(card_entity, transform)| {
        if card_entity.stun_count > 0 {
            let mut stun_indicator_tuple = stun_indicator_list.pop().unwrap();
            stun_indicator_tuple.0.translation = transform.translation.clone();
            stun_indicator_tuple.1.is_visible = true;
        }
    });
    for (_, mut visibility) in stun_indicator_list {
        visibility.is_visible = false;
    }
}
