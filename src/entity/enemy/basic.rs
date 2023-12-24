use bevy::prelude::*;
use crate::entity::entity::{EntityComponent, Entity, spawn_enemy, SpriteSheet};
use crate::screen::ScreenResolution;


#[derive(Component)]
pub struct Basic {
    pub(crate) entity_component: EntityComponent,
    pub(crate) movement: Movements
}

pub enum Movements {
    StraightLR,
    StraightRL
}

impl Entity for Basic {
    fn component(&mut self) -> EntityComponent {
        self.entity_component.clone()
    }
}

impl Basic {
    pub(crate) fn spawn(
        commands: &mut Commands,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        asset_server: &mut Res<AssetServer>,
        screen: &ScreenResolution
    ) {
        spawn_enemy(
            commands,
            texture_atlases,
            asset_server,
            screen,
            0,
            Basic {
                entity_component:EntityComponent{
                    x: (screen.width/2.) - 23.0,
                    y: 0.0,
                    z: 100.0,
                    speed: 200.0,
                    fire_speed: Timer::from_seconds(1.0, TimerMode::Repeating),
                    sprite_sheet: SpriteSheet::new(
                        "entity/enemy1.png".to_string(),
                        Vec2::new(23.0, 27.0),
                        1,
                        1
                    )
                },
                movement: Movements::StraightRL
            }
        )
    }
    pub(crate) fn animate(
    time: Res<Time>,
    mut query : Query<(&mut Transform, &mut Basic), With<Basic>>,
    ) {
        for (mut transform, mut entity) in &mut query {

            match entity.movement {
                Movements::StraightRL => {
                    transform.translation.x += -1.0 * entity.entity_component.speed * time.delta_seconds();
                },
                Movements::StraightLR => {
                    transform.translation.x += 1.0 * entity.entity_component.speed * time.delta_seconds();
                }
            }
            entity.entity_component.x = transform.translation.x;
            entity.entity_component.y = transform.translation.y;
        };
    }
}