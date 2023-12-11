use std::string::ToString;
use bevy::math::Vec2;
use bevy::prelude::{Component, DetectChanges, KeyCode, Query, Res, Time};
use crate::entity::entity_manager::SpriteSheet;
use bevy::input::{Input};
use bevy::time::Timer;
use bevy::ui::UiTextureAtlasImage;

#[derive(Component)]
pub struct Player {
    pub (crate) sprite_sheet: SpriteSheet,
}

const FILE_PATH:&'static str = "entity/player_plane.png";
const TILE_SIZE:Vec2 = Vec2::new(64., 64.,);
const COL:usize = 4;
const ROW:usize = 2;



impl Player {
    pub(crate) fn new() -> Player {
        Player {
            sprite_sheet: SpriteSheet { file_path: FILE_PATH.to_string(), tile_size: TILE_SIZE, col: COL, row: ROW}
        }
    }
}

pub fn keyboard_events(
    keyboard_input: Res<Input<KeyCode>>,
    mut query : Query<(&mut UiTextureAtlasImage)>
) {
    if keyboard_input.pressed(KeyCode::Up){
        for mut atlas_image in &mut query {
            atlas_image.index = 1;
        }
    }
    else if keyboard_input.pressed(KeyCode::Down){
        for mut atlas_image in &mut query {
            atlas_image.index = 3;
        }
    }
    else  {
        for mut atlas_image in &mut query {
            atlas_image.index = 0;
        }
    }
}