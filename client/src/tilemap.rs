use bevy::prelude::*;

use crate::game::card_interactions::{AttackIndicator, MoveIndicator};

pub struct TileSize(pub f32);

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileSize(
            app.world
                .get_resource::<Windows>()
                .unwrap()
                .get_primary()
                .unwrap()
                .height()
                / 9.,
        ))
        .add_startup_system(spawn_tilemap_bg);
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
                        Color::hex("f6e1b1").unwrap()
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
                            100.,
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
