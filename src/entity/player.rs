use bevy::asset::{Assets, AssetServer};
use bevy::math::{Vec2};
use bevy::prelude::{Commands, Component, Res, ResMut, TextureAtlas};
use bevy::time::{Timer, TimerMode};
use crate::entity::entity::{Entity, EntityComponent, spawn_player, SpriteSheet};
use crate::state::screen::{ScreenResolution};

#[derive(Component)]
pub struct Player {
    pub(crate) entity_component: EntityComponent,
}

impl Entity for Player {
    fn component(&mut self) -> EntityComponent {
        self.entity_component.clone()
    }

    fn fire_speed(&mut self) -> Timer {
        self.entity_component.fire_speed.clone()
    }

    fn set_position(&mut self, x: f32, y: f32) {
        self.entity_component.x = x;
        self.entity_component.y = y;
    }

    fn get_position(&mut self) -> (f32, f32) {
        (self.entity_component.x, self.entity_component.y)
    }

}

pub fn spawn(
    commands: &mut Commands,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: &mut Res<AssetServer>,
    screen: &ScreenResolution
) {
    spawn_player(
        commands,
        texture_atlases,
        asset_server,
        screen,
        0,
        Player {
            entity_component: EntityComponent {
                x: -((screen.width/2.)-(35.*screen.scale)),
                y: 0.0,
                z: 100.,
                fire_speed: Timer::from_seconds(0.2, TimerMode::Repeating),
                sprite_sheet: SpriteSheet::new(
                    "entity/player_plane.png".to_string(),
                    Vec2::new(64., 64.,),
                    4,
                    2
                )
            }
        }

     );
}