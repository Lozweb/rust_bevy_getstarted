use bevy::prelude::*;
use crate::background::nebuleuse::{Nebuleuse, spawn_nebuleuse};
use crate::background::star::{spawn_star, Star};
use crate::entity::screen::CURRENT_MODE;
use crate::states::states::{despawn_screen, GameState};
pub struct GameBackgroundPlugin;

#[derive(Component)]
struct OnGameScreen;

impl Plugin for GameBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), background_setup)
            .add_systems(Update, (stars_animation, background_animation))
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

fn background_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let screen = unsafe {CURRENT_MODE.get_resolution()};
    spawn_nebuleuse(&mut commands, &mut asset_server, 0.);
    spawn_nebuleuse(&mut commands, &mut asset_server, screen.width);

    let max_stars = (200.*screen.scale) as i32;

    for _ in 0..max_stars {
        spawn_star(&mut commands, &mut meshes, &mut materials, false)
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

