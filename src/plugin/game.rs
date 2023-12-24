use bevy::prelude::*;
use crate::entity::player::*;
use crate::entity::enemy::basic::Enemy;
use crate::screen::CURRENT_MODE;
use crate::states::{GameInitState, GameState};

pub struct GamePlugin;

#[derive(Component)]
pub struct OnGameScreen;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, (
                player_capabilities::player_movement,
                player_attack::projectil_animation
            ).run_if(in_state(GameState::Game)));
    }
}

fn game_setup(
    mut commands: Commands,
    mut game_init_sate: ResMut<NextState<GameInitState>>,
    current_state: Res<State<GameInitState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut asset_server: Res<AssetServer>,
){
    if current_state.get() == &GameInitState::Starting {

        let screen = unsafe {CURRENT_MODE.get_resolution()};

        player_character::Player::spawn(&mut commands, &mut texture_atlases, &mut asset_server, &screen);
        Enemy::spawn(&mut commands, &mut texture_atlases, &mut asset_server, &screen);

        game_init_sate.set(GameInitState::InProgress);
    }
}