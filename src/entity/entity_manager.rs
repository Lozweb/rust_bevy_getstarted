use bevy::asset::{AssetServer};
use bevy::math::Vec2;
use bevy::prelude::{Res, TextureAtlas};

pub struct SpriteSheet {
    pub(crate) file_path: String,
    pub(crate) tile_size: Vec2,
    pub(crate) col: usize,
    pub(crate) row: usize
}

const PLAYER_FILE:&'static str = "entity/player_plane.png";
const TILE_SIZE:Vec2 = Vec2::new(64., 64.,);
const COL:usize = 4;
const ROW:usize = 2;

impl  SpriteSheet {
    pub (crate) fn player() -> SpriteSheet {
        SpriteSheet {
            file_path: PLAYER_FILE.to_string(),
            tile_size: TILE_SIZE,
            col: COL,
            row: ROW
        }
    }
}

pub fn texture_atlas(
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


