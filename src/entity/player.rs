use std::string::ToString;
use bevy::math::Vec2;
use bevy::prelude::{Component, KeyCode, Query, Res, Time, Transform};
use crate::entity::entity_manager::Tile;
use bevy::input::{Input};

#[derive(Component)]
pub struct Player {
    pub (crate) tile_set: Tile,
    pub (crate) animation_layer: AnimationLayer
}

#[derive(Component)]
pub struct AnimationLayer {
    pub (crate) default: [usize; 1],
    pub (crate) to_up: [usize; 2],
    pub (crate) to_down: [usize; 2],
}

const PATH:&'static str = "entity/player_plane.png";
const SIZE:Vec2 = Vec2::new(64.,64.,);
const COL:usize = 4;
const ROW:usize = 2;
const DEFAULT: [usize; 1] = [0];
const TO_UP: [usize; 2] = [1,2];
const TO_DOWN: [usize; 2] = [3,4];



impl Player {
    pub(crate) fn new() -> Player {
        Player {
            tile_set: Tile {path:PATH.to_string(), size: SIZE, col: COL, row: ROW},
            animation_layer: AnimationLayer{default: DEFAULT, to_up: TO_UP, to_down: TO_DOWN}
        }
    }
}

pub fn keyboard_events(
    keyboard_input: Res<Input<KeyCode>>,
    mut query : Query<(&Player, &mut Transform)>
) {
    for(player, mut transform) in &mut query {
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y += 1.0;
        }
    }
}