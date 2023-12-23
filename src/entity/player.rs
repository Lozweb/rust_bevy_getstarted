use bevy::asset::{Assets, AssetServer};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, Component, default, Res, ResMut, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite, Transform};
use bevy::time::{Timer, TimerMode};
use crate::states::game::OnGameScreen;
use crate::states::screen::{ScreenResolution};

#[derive(Component)]
pub struct Player {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) fire_speed: Timer
}

impl Player {
    pub fn set_position(&mut self, x:f32, y:f32){
        self.x = x;
        self.y = y;
    }
}

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
    pub(crate) fn sprite_sheet() -> SpriteSheet {
        SpriteSheet {
            file_path: PLAYER_FILE.to_string(),
            tile_size: TILE_SIZE,
            col: COL,
            row: ROW
        }
    }
}

pub fn player_texture(
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
    screen: &ScreenResolution
) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlases.add(player_texture(&asset_server, &SpriteSheet::sprite_sheet())),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(-((screen.width/2.)-(35.*screen.scale)),0.0,100.0).with_scale(Vec3::splat(screen.scale)),
            ..default()
        },
        Player{
            x: -((screen.width/2.)-(35.*screen.scale)),
            y: 0.0,
            fire_speed: Timer::from_seconds(0.2, TimerMode::Repeating)
        },
        OnGameScreen
    ));
}