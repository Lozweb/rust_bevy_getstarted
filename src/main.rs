use bevy::prelude::*;
use crate::entity::entity_manager::{SpriteSheet, texture_atlas};
use crate::entity::player::{Player, player_movement};

mod entity {
    pub mod entity_manager;
    pub mod player;
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    commands.spawn(Camera2dBundle::default());

    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlases.add(texture_atlas(&asset_server, &SpriteSheet::player())),
        sprite: TextureAtlasSprite::new(0),
        transform: Transform::from_xyz(100.0,100.0,0.0),
        ..default()
    }, Player{}
    ));
}

