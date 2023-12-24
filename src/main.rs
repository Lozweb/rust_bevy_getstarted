use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::window::WindowTheme;
use state::screen::get_current_screen_resolution;
use crate::state::game::GamePlugin;
use crate::state::main_menu::MenuPlugin;
use crate::state::paused::PausedMenuPlugin;
use crate::state::states::{GameInitState, GameState};

mod level {
    pub mod level_manager;
}

mod background{
    pub mod nebuleuse;
    pub mod star;
}
mod entity {
    pub mod player;
    pub mod player_attack;
    pub mod player_capabilities;
    pub mod entity;
    pub mod enemy {
        pub mod basic;
    }
}

mod state {
    pub mod main_menu;
    pub mod menu_element;
    pub mod game;
    pub mod states;
    pub mod paused;
    pub mod screen;
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
        .add_state::<GameInitState>()
        .add_systems(Startup, setup)
        .add_plugins((MenuPlugin, GamePlugin, PausedMenuPlugin))
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

