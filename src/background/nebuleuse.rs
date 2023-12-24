use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{Color, Commands, Component, default, Res, Sprite, SpriteBundle, Transform};
use crate::state::game::OnGameScreen;

#[derive(Component)]
pub struct Nebuleuse {
    pub(crate) texture: &'static str,
    pub(crate) sprite: Sprite,
}

impl Nebuleuse {
    pub fn new() -> Nebuleuse {
        Nebuleuse {
            texture: "background/nebuleuse.png",
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.1),
                ..default()
            },
        }
    }
}

pub fn spawn_nebuleuse(
    commands: &mut Commands,
    asset_server: &mut Res<AssetServer>,
    x_position: f32
) {
    let nebuleuse = Nebuleuse::new();
    commands.spawn((
        SpriteBundle{
            sprite : nebuleuse.sprite,
            transform: Transform::from_translation(Vec3::new(x_position, 0., 0.)),
            texture: asset_server.load(nebuleuse.texture),
            ..default()
        }, Nebuleuse::new(),
        OnGameScreen
    ));
}