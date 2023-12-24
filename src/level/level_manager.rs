use bevy::prelude::*;
use crate::state::states::GameState;


pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), level_setup);
    }
}

fn level_setup() {

}