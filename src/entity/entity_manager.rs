use bevy::asset::{AssetServer};
use bevy::math::Vec2;
use bevy::prelude::{Res, TextureAtlas};
pub struct Tile {
    pub(crate) path: String,
    pub(crate) size: Vec2,
    pub(crate) col: usize,
    pub(crate) row: usize
}

pub fn texture_atlas(asset_server: Res<AssetServer>, tile: Tile) ->  TextureAtlas {
    TextureAtlas::from_grid(asset_server.load(tile.path),
    tile.size,
    tile.col,
    tile.row,
    None,
    None
    )
}