use bevy::prelude::*;
use crate::background::nebuleuse::{Nebuleuse, spawn_nebuleuse};
use crate::background::star::{Star, spawn_star};
use crate::entity::player_capabilities::player_movement;
use crate::entity::player::spawn_player;
use crate::entity::player_attack::Projectil;
use crate::states::screen::CURRENT_MODE;
use crate::states::states::{GameInitState, GameState};

pub struct GamePlugin;

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, (player_movement, projectil_animation, background_animation, stars_animation).run_if(in_state(GameState::Game)));
    }
}

fn game_setup(
    mut commands: Commands,
    mut game_init_sate: ResMut<NextState<GameInitState>>,
    current_state: Res<State<GameInitState>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
){
    if current_state.get() == &GameInitState::Starting {

        let screen = unsafe {CURRENT_MODE.get_resolution()};

        spawn_player(&mut commands, texture_atlases, &mut asset_server, &screen);

        spawn_nebuleuse(&mut commands, &mut asset_server, 0.);
        spawn_nebuleuse(&mut commands, &mut asset_server, screen.width);

        let max_stars = (200.*screen.scale) as i32;

        for _ in 0..max_stars {
            spawn_star(&mut commands, &mut meshes, &mut materials, false)
        }
        game_init_sate.set(GameInitState::InProgress);
    }
}

fn background_animation(
    time: Res<Time>,
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut query: Query<(&mut Transform, Entity), With<Nebuleuse>>
){
    let screen = unsafe {CURRENT_MODE.get_resolution()};

    for (mut transform, entity) in &mut query{
        transform.translation.x -= 1. * 20. * time.delta_seconds();

        if transform.translation.x <= -screen.width {
            commands.entity(entity).despawn();
            spawn_nebuleuse(&mut commands, &mut asset_server, screen.width);
        }
    }
}

fn stars_animation(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut stars : Query<(&mut Transform, &mut Star, Entity), With<Star>>
){
    let screen = unsafe {CURRENT_MODE.get_resolution()};

    for (mut transform, star, entity) in &mut stars {
        transform.translation.x += -1. * ((star.speed * star.size)*2.) * time.delta_seconds();

        if transform.translation.x <= -(screen.width/2.) {
            commands.entity(entity).despawn();
            spawn_star(&mut commands, &mut meshes, &mut materials, true);
        }
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

        if transform.translation.x >= screen.width {
            commands.entity(entity).despawn();
        }
    }
}