use crate::tilemap::TileSize;
use bevy::prelude::*;

pub struct OwnershipIndicatorPlugin;

impl Plugin for OwnershipIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

#[derive(Component)]
pub struct OpponentOwned;
#[derive(Component)]
pub struct SelfOwned;

fn setup(mut commands: Commands, tile_size: Res<TileSize>) {
    for i in 0..12 {
        let entity = commands
            .spawn(SpriteBundle {
                transform: Transform::from_xyz(1000000.0, 0.0, 400.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(tile_size.0 * 0.9)),
                    color: Color::hex(if i % 2 == 0 { "2b8fc4" } else { "e0828a" }).unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();
        if i % 2 == 0 {
            commands.entity(entity).insert(SelfOwned);
        } else {
            commands.entity(entity).insert(OpponentOwned);
        }
    }
}
