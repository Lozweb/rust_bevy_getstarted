use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::window::WindowTheme;
use crate::background::game_background::GameBackgroundPlugin;
use crate::entity::screen::get_current_screen_resolution;
use crate::states::game::GamePlugin;
use crate::states::main_menu::{MenuPlugin};
use crate::states::states::GameState;

mod background{
    pub mod game_background;
    pub mod nebuleuse;
    pub mod star;
}
mod entity {
    pub mod entity_manager;
    pub mod player;
    pub mod screen;
}
mod states {
    pub mod main_menu;
    pub mod menu_element;
    pub mod game;
    pub mod states;
}
fn main() {
    let default_resolution = unsafe { get_current_screen_resolution() };
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (default_resolution.width, default_resolution.height).into(),
                        title: "Space shooter".to_string(),
                        resizable: false,
                        enabled_buttons: bevy::window::EnabledButtons {
                            minimize: true,
                            maximize: false,
                            close: true,
                        },
                        window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
        )
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((MenuPlugin, GamePlugin, GameBackgroundPlugin))
        .run();
}

fn setup(mut commands: Commands, ){
    commands.spawn((
        Camera2dBundle {
            camera: Camera{
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
    ));
}

