use bevy::asset::{Assets, AssetServer};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, Component, Res, ResMut, SpriteSheetBundle, TextureAtlas, Timer, Transform};
use bevy::sprite::TextureAtlasSprite;
use bevy::utils::default;
use crate::entity::player::Player;
use crate::state::game::OnGameScreen;
use crate::state::screen::ScreenResolution;

#[derive(Component, Clone)]
pub struct EntityComponent {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) fire_speed: Timer,
    pub(crate) sprite_sheet: SpriteSheet
}

pub trait Entity {
    fn component(&mut self) -> EntityComponent;

    fn fire_speed(&mut self) -> Timer {self.component().fire_speed}

    fn set_position(&mut self, x:f32, y:f32)-> () {
        self.component().x = x;
        self.component().y = y;
    }
    fn get_position(&mut self) -> (f32, f32) {
        (self.component().x, self.component().y)
    }

}

#[derive(Clone)]
pub struct SpriteSheet {
    pub(crate) file_path: String,
    pub(crate) tile_size: Vec2,
    pub(crate) col: usize,
    pub(crate) row: usize
}

impl  SpriteSheet {
    pub(crate) fn new(file_path: String, tile_size: Vec2, col: usize, row: usize) -> SpriteSheet {
        SpriteSheet {
            file_path,
            tile_size,
            col,
            row
        }
    }
}

pub fn texture(
    asset_server: &Res<AssetServer>,
    sprite_sheet: &SpriteSheet
) ->  TextureAtlas {

    TextureAtlas::from_grid(
        asset_server.load(&sprite_sheet.file_path),
        sprite_sheet.tile_size,
        sprite_sheet.col,
        sprite_sheet.row,
        None,
        None
    )
}

pub fn spawn_player(
    commands: &mut Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: &mut Res<AssetServer>,
    screen: &ScreenResolution,
    texture_atlas_index: usize,
    mut entity: Player
) {

    let entity_component = entity.component();

    commands.spawn( (
        SpriteSheetBundle {
            texture_atlas: texture_atlases.add(texture(&asset_server, &entity_component.sprite_sheet)),
            sprite: TextureAtlasSprite::new(texture_atlas_index),
            transform: Transform::from_xyz(entity_component.x, entity_component.y, entity_component.z).with_scale(Vec3::splat(screen.scale)),
            ..default()
        },
        entity,
        OnGameScreen
    ));

}