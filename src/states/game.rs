use bevy::prelude::*;
use crate::entity::entity_manager::{SpriteSheet, texture_atlas};
use crate::entity::player::{Player, player_movement};
use crate::entity::screen::CURRENT_MODE;
use crate::states::states::{despawn_screen, GameState};

pub struct GamePlugin;

#[derive(Component)]
struct OnGameScreen;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, player_movement.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    let screen = unsafe {CURRENT_MODE.get_resolution()};
    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlases.add(texture_atlas(&asset_server, &SpriteSheet::player())),
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_xyz(-((screen.width/2.)-(35.*screen.scale)),0.0,100.0).with_scale(Vec3::splat(screen.scale)),
        ..default()
    }, Player{}
    ));
}