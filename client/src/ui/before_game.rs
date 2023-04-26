use bevy::prelude::*;
use super::*;

pub struct BeforeGamePlugin;

impl Plugin for BeforeGamePlugin{
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::PreparingForGame).with_system(build_ui))
            .add_system_set(SystemSet::on_enter(GameState::PreparingForGame).with_system(send_deck_packet)) 
            .add_system_set(SystemSet::on_exit(GameState::PreparingForGame).with_system(destroy_ui));
    }
}

fn send_deck_packet(
    settings: Res<Settings>,
    mut queue_out: ResMut<QueueOut>,
    mut commands: Commands,
){
    queue_out.0.lock().unwrap().push_back(ClientMessage::PlayerInfo(settings.username.clone(), settings.deck.clone()));
    commands.insert_resource(Deck(settings.deck.clone()));
}

