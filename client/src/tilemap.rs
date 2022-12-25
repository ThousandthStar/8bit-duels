use bevy::prelude::*;

use crate::card_interactions::{AttackIndicator, MoveIndicator};
use common::card::CardEntity;
use std::collections::HashMap;

#[derive(Resource)]
pub struct TileSize(pub f32);

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tilemap_bg)
            .add_startup_system_to_stage(StartupStage::PreStartup, add_tile_size_res)
            .add_system(position_sprites)
            .add_system(load_card_sprites);
    }
}

fn add_tile_size_res(mut commands: Commands, windows: Res<Windows>) {
    commands.insert_resource(TileSize(windows.get_primary().unwrap().height() / 9.));
}

#[derive(Resource)]
pub(crate) struct CardSprites(
    pub(crate) Handle<TextureAtlas>,
    pub(crate) HashMap<String, usize>,
);

fn load_card_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_sheet = asset_server.load("sprite_sheet.png");
    let atlas: TextureAtlas =
        TextureAtlas::from_grid(sprite_sheet, Vec2::splat(32.0), 1, 2, None, None);

    let atlas_handle = texture_atlases.add(atlas);
    let mut card_sprite_map = HashMap::new();
    card_sprite_map.insert("skeleton".to_string(), 0);
    card_sprite_map.insert("reaper".to_string(), 1);

    commands.insert_resource(CardSprites(atlas_handle, card_sprite_map));
}

fn position_sprites(
    mut query: Query<(&mut Transform, &CardEntity)>,
    windows: Res<Windows>,
    tile_size: Res<TileSize>,
) {
    let window = windows.get_primary().unwrap();
    // parentheses only for clarity (they are unnecessary)
    let start_y = (window.height() / 2.) - (tile_size.0 / 2.);
    let start_x = (-window.width() / 2.) + (tile_size.0 / 2.) + (window.width() / 3.);

    for (mut transform, card_entity) in query.iter_mut() {
        transform.translation.x = start_x + (card_entity.get_x_pos() as f32 * tile_size.0);
        transform.translation.y = start_y - (card_entity.get_y_pos() as f32 * tile_size.0);
        transform.translation.z = 150.;
    }
}

fn spawn_tilemap_bg(
    mut commands: Commands,
    windows: Res<Windows>,
    tile_size: Res<TileSize>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.get_primary().unwrap();
    let one_third_window = window.width() / 3.;
    let start_y = (window.height() / 2.) - (tile_size.0 / 2.);
    let start_x = (-window.width() / 2.) + (tile_size.0 / 2.);
    for i in 0..5 {
        for j in 0..9 {
            commands.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(
                    start_x + one_third_window + (i as f32 * tile_size.0),
                    start_y - (j as f32 * tile_size.0),
                    100.,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(tile_size.0)),
                    color: if (i + (j * 5)) % 2 == 0 {
                        Color::hex("ffa19e").unwrap()
                    } else {
                        Color::hex("9eb5c0").unwrap()
                    },
                    ..Default::default()
                },
                ..Default::default()
            });
            for l in 0..2 {
                let spawned_entity = commands
                    .spawn_bundle(SpriteBundle {
                        transform: Transform::from_xyz(
                            start_x + one_third_window + (i as f32 * tile_size.0),
                            start_y - (j as f32 * tile_size.0),
                            200.,
                        ),
                        sprite: Sprite {
                            color: Color::rgba(0.8, 0.8, 0.8, 0.62),
                            custom_size: Some(Vec2::splat(tile_size.0 * 0.8)),

                            ..Default::default()
                        },
                        visibility: Visibility { is_visible: false },
                        texture: asset_server.load(if l == 1 {
                            "move_indicator.png"
                        } else {
                            "attack_indicator.png"
                        }),
                        ..Default::default()
                    })
                    .id();
                if l == 1 {
                    commands.entity(spawned_entity).insert(MoveIndicator(i, j));
                } else {
                    commands
                        .entity(spawned_entity)
                        .insert(AttackIndicator(i, j, 0.));
                }
            }
        }
    }
}
