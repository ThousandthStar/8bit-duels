use bevy::prelude::*;

use crate::GameState;

#[derive(Resource)]
pub struct Spirits(pub i32);

#[derive(Resource)]
pub struct Pawns(pub i32);

pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Spirits(8))
            .insert_resource(Pawns(6))
            .add_system_set(SystemSet::on_enter(GameState::Waiting).with_system(reset_currencies));
    }
}

fn reset_currencies(mut spirits: ResMut<Spirits>, mut pawns: ResMut<Pawns>) {
    spirits.0 = 8;
    pawns.0 = 6;
}
