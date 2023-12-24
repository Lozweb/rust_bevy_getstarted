use bevy::prelude::*;
use crate::entity::enemy::basic::{Basic};
use crate::screen::CURRENT_MODE;
use crate::states::{GameInitState, GameState};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameInitState::LevelLoading), level_setup)
            .add_systems(Update, (enemy_animate).run_if(in_state(GameState::Game)));
    }
}

fn level_setup(
    mut commands: Commands,
    mut game_init_sate: ResMut<NextState<GameInitState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut asset_server: Res<AssetServer>,
) {
    println!("level 1 setup");
    let screen = unsafe {CURRENT_MODE.get_resolution()};
    Basic::spawn(&mut commands, &mut texture_atlases, &mut asset_server, &screen);
    game_init_sate.set(GameInitState::InGame);
}

pub fn enemy_animate(
    time: Res<Time>,
    basics : Query<(&mut Transform, &mut Basic), With<Basic>>,
) {
    Basic::animate(time, basics);
}