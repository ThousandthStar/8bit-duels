use bevy::prelude::*;

use crate::{
    animations::{AttackAnimation, MovementAnimation},
    card_interactions::{AttackIndicator, MoveIndicator},
    ownership_indicator::{OpponentOwned, SelfOwned},
    IsPlayer1,
};
use common::card::CardEntity;
use std::collections::HashMap;

#[derive(Resource)]
pub struct TileSize(pub f32);

#[derive(Component)]
pub struct InstantMove;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_tiles)
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
        TextureAtlas::from_grid(sprite_sheet, Vec2::splat(32.0), 1, 4, None, None);

    let atlas_handle = texture_atlases.add(atlas);
    let mut card_sprite_map = HashMap::new();
    card_sprite_map.insert("reaper".to_string(), 0);
    card_sprite_map.insert("skeleton".to_string(), 1);
    card_sprite_map.insert("kraken".to_string(), 2);
    card_sprite_map.insert("spider".to_string(), 3);

    commands.insert_resource(CardSprites(atlas_handle, card_sprite_map));
}

fn position_sprites(
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &CardEntity,
            Option<&InstantMove>,
            Option<&MovementAnimation>,
            Option<&AttackAnimation>,
        ),
        (Without<SelfOwned>, Without<OpponentOwned>),
    >,
    mut self_ownership_q: Query<
        (&mut Transform, &mut Visibility),
        (With<SelfOwned>, Without<OpponentOwned>),
    >,
    mut opponent_ownership_q: Query<
        (&mut Transform, &mut Visibility),
        (With<OpponentOwned>, Without<SelfOwned>),
    >,
    windows: Res<Windows>,
    tile_size: Res<TileSize>,
    is_player_1: Res<IsPlayer1>,
    mut commands: Commands,
) {
    let window = windows.get_primary().unwrap();
    // parentheses only for clarity (they are unnecessary)
    let start_y = (window.height() / 2.) - (tile_size.0 / 2.);
    let start_x = (-window.width() / 2.) + (tile_size.0 / 2.) + (window.width() / 3.);

    let mut self_ownership_list: Vec<(Mut<Transform>, Mut<Visibility>)> =
        self_ownership_q.iter_mut().collect();
    let mut opponent_ownership_list: Vec<(Mut<Transform>, Mut<Visibility>)> =
        opponent_ownership_q.iter_mut().collect();

    for (entity, mut transform, card_entity, instant_move_opt, move_anim_opt, atck_anim_opt) in
        query.iter_mut()
    {
        transform.translation.z = 500.;
        let target = Vec2::new(
            start_x + (card_entity.get_x_pos() as f32 * tile_size.0),
            start_y - (card_entity.get_y_pos() as f32 * tile_size.0),
        );
        if instant_move_opt.is_some() {
            transform.translation.x = target.x;
            transform.translation.y = target.y;
            commands.entity(entity).remove::<InstantMove>();
        } else if (transform.translation.x != target.x || transform.translation.y != target.y)
            && !atck_anim_opt.is_some()
        {
            commands.entity(entity).insert(MovementAnimation { target });
        }

        let (mut indicator_transform, mut visibility) = self_ownership_list.pop().unwrap();
        if card_entity.is_owned_by_p1() == is_player_1.0
            && !move_anim_opt.is_some()
            && !atck_anim_opt.is_some()
        {
            indicator_transform.translation = transform.translation;
            indicator_transform.translation.z = 400.0;
            visibility.is_visible = true;
        } else {
            visibility.is_visible = false;
        }
        let (mut indicator_transform, mut visibility) = opponent_ownership_list.pop().unwrap();
        if card_entity.is_owned_by_p1() != is_player_1.0
            && !move_anim_opt.is_some()
            && !atck_anim_opt.is_some()
        {
            indicator_transform.translation = transform.translation;
            indicator_transform.translation.z = 400.0;
            visibility.is_visible = true;
        } else {
            visibility.is_visible = false;
        }
    }
}

#[derive(Component)]
pub struct Tile;

fn spawn_tiles(
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
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_xyz(
                        start_x + one_third_window + (i as f32 * tile_size.0),
                        start_y - (j as f32 * tile_size.0),
                        0.,
                    ),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(tile_size.0)),
                        color: if (i + (j * 5)) % 2 == 0 {
                            Color::hex("f2f2f2").unwrap()
                        } else {
                            Color::hex("ffffff").unwrap()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Tile);
            for l in 0..2 {
                let spawned_entity = commands
                    .spawn_bundle(SpriteBundle {
                        transform: Transform::from_xyz(
                            start_x + one_third_window + (i as f32 * tile_size.0),
                            start_y - (j as f32 * tile_size.0),
                            250.,
                        ),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(
                                tile_size.0 * if l == 1 { 0.80 } else { 1.0 },
                            )),
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
