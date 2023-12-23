use bevy::asset::Assets;
use bevy::prelude::{ColorMaterial, Commands, KeyCode, Mesh, NextState, Query, Res, ResMut, Time, Transform, With};
use bevy::input::{Input};
use bevy::math::Vec3;
use bevy::sprite::TextureAtlasSprite;
use crate::entity::player::Player;
use crate::entity::player_attack::spwan_projectil;
use crate::states::screen::CURRENT_MODE;
use crate::states::paused::MenuPausedState;
use crate::states::states::GameState;

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuPausedState>>,
    mut query : Query<(&mut Transform, &mut TextureAtlasSprite, &mut Player), With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let screen = unsafe {CURRENT_MODE.get_resolution()};
    let mut x_direction = 0.0;
    let mut y_direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        x_direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right){
        x_direction += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        y_direction += 1.0;
    }
    if  keyboard_input.pressed(KeyCode::Down) {
        y_direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Escape) {
        game_state.set(GameState::Paused);
        menu_state.set(MenuPausedState::Main);
    }

    for (mut transform, mut sprite, mut player) in &mut query {
        if y_direction > 0.0 { sprite.index = 1; }
        if y_direction < 0.0 { sprite.index = 3; }
        if y_direction == 0.0 { sprite.index = 0; }

        if transform.translation.x <= -((screen.width/2.)-(30.*screen.scale)) { x_direction = 1.0; }
        if transform.translation.x >= ((screen.width/2.)-(30.*screen.scale)) { x_direction = -1.0; }
        if transform.translation.y >= ((screen.height/2.)-(20.*screen.scale)) { y_direction = -1.0; }
        if transform.translation.y <= -((screen.height/2.)-(20.*screen.scale)) { y_direction = 1.0; }

        transform.translation.x += x_direction * 200.0 * time.delta_seconds();
        transform.translation.y += y_direction * 200.0 * time.delta_seconds();
        player.set_position(transform.translation.x, transform.translation.y)
    }

    if keyboard_input.pressed(KeyCode::Space){

        for (_, _, mut player ) in &mut query{

            player.fire_speed.tick(time.delta());

            if player.fire_speed.just_finished(){
                spwan_projectil(&mut commands, &mut meshes, &mut materials, Vec3::new(player.x, player.y, 100.));
            }

        }
    }
}