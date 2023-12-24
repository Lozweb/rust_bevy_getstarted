use bevy::prelude::*;
use crate::entity::player_capabilities::player_movement;
use crate::entity::player::*;
use crate::entity::player_attack::Projectil;
use crate::entity::enemy::basic::Enemy;
use crate::state::screen::CURRENT_MODE;
use crate::state::states::{GameInitState, GameState};

pub struct GamePlugin;

#[derive(Component)]
pub struct OnGameScreen;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, (player_movement, projectil_animation).run_if(in_state(GameState::Game)));
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

        Player::spawn(&mut commands, &mut texture_atlases, &mut asset_server, &screen);
        Enemy::spawn(&mut commands, &mut texture_atlases, &mut asset_server, &screen);

        game_init_sate.set(GameInitState::InProgress);
    }
}


fn projectil_animation(
    time: Res<Time>,
    mut commands: Commands,
    mut projectils : Query<(&mut Transform, &mut Projectil, Entity), With<Projectil>>
) {
    let screen = unsafe {CURRENT_MODE.get_resolution()};

    for (mut transform, projectil, entity) in &mut projectils {

        transform.translation.x += 1. + projectil.speed * time.delta_seconds();

        // despawn on right screen
        if transform.translation.x >= screen.width {
            commands.entity(entity).despawn();
        }
    }
}