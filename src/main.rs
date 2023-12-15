use bevy::prelude::*;
use crate::states::game::GamePlugin;
use crate::states::main_menu::{MenuPlugin};
use crate::states::states::GameState;

mod entity {
    pub mod entity_manager;
    pub mod player;
}
mod states {
    pub mod main_menu;
    pub mod menu_element;
    pub mod game;
    pub mod states;
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((MenuPlugin, GamePlugin))
        .run();
}
fn setup(mut commands: Commands, ){
    commands.spawn(Camera2dBundle::default());
}

