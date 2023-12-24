use bevy::prelude::*;
use crate::entity::entity::{EntityComponent, Entity, spawn_enemy, SpriteSheet};
use crate::state::screen::ScreenResolution;


#[derive(Component)]
pub struct  Enemy {
    pub(crate) entity_component: EntityComponent,
}

impl Entity for Enemy {
    fn component(&mut self) -> EntityComponent {
        self.entity_component.clone()
    }
}

impl Enemy {
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
            Enemy {
                entity_component:EntityComponent{
                    x: (screen.width/2.) - 23.0,
                    y: 0.0,
                    z: 100.0,
                    fire_speed: Timer::from_seconds(1.0, TimerMode::Repeating),
                    sprite_sheet: SpriteSheet::new(
                        "entity/enemy1.png".to_string(),
                        Vec2::new(23.0, 27.0),
                        1,
                        1
                    )
                }
            }
        )
    }
}