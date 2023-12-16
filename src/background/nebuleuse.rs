use bevy::asset::AssetServer;
use bevy::math::Vec3;
use bevy::prelude::{Color, Commands, Component, default, Res, Sprite, SpriteBundle, Transform};

#[derive(Component)]
pub struct Nebuleuse {
    pub(crate) texture: &'static str,
    pub(crate) sprite: Sprite,
    pub(crate) position: Vec3
}

impl Nebuleuse {
    pub fn new() -> Nebuleuse {
        Nebuleuse {
            texture: "background/nebuleuse.png",
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.1),
                ..default()
            },
            position: Vec3::new(0., 0., 0.)
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
            transform: Transform::from_translation(Vec3::new(x_position, 0., 0.)).with_scale(Vec3::new(0.7, 0.7, 0.7)),
            texture: asset_server.load(nebuleuse.texture),
            ..default()
        }, Nebuleuse::new()
    ));
}