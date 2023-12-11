use bevy::prelude::*;
use crate::entity::entity_manager::texture_atlas;
use crate::entity::player::{keyboard_events, Player};

mod entity {
    pub mod entity_manager;
    pub mod player;
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_events)
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    commands.spawn(Camera2dBundle::default());

    let player =  Player::new();
    let texture_atlas_handle =
        texture_atlases.add(
            texture_atlas(asset_server, player.tile_set));

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(player.animation_layer.default[0]),
            ..default()
        },
        player.animation_layer,
        AnimationTimer(Timer::from_seconds(0.8, TimerMode::Repeating)),
    ));
}

