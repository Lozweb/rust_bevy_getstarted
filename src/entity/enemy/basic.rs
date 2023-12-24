use bevy::prelude::*;
use crate::entity::entity::{EntityComponent, Entity, SpriteSheet};


#[derive(Component)]
pub enum Enemy {
    Basic
}

impl Entity for Enemy {
    fn component(&mut self) -> EntityComponent {
        match &self {
            Enemy::Basic => EntityComponent {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                fire_speed: Timer::from_seconds(1.0, TimerMode::Repeating),
                sprite_sheet: SpriteSheet::new(
                    "entity/enemy1.png".to_string(),
                    Vec2::new(23.0, 27.0),
                    1,
                    1
                )
            }
        }
    }
}