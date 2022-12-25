use bevy::prelude::*;

#[derive(Resource)]
pub struct Spirits(pub i32);

#[derive(Resource)]
pub struct Pawns(pub i32);

pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Spirits(100)).insert_resource(Pawns(6));
    }
}
