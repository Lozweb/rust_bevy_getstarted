use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::window::WindowTheme;
use screen::get_current_screen_resolution;

pub mod screen;
pub mod states;

pub mod menu {
    pub mod menu_element;
}
mod background{
    pub mod nebuleuse;
    pub mod star;
}
mod entity {
    pub mod entity;
    pub mod enemy {
        pub mod basic;
    }
    pub mod player {
        pub mod player_character;
        pub mod player_attack;
        pub mod player_capabilities;
    }
}

mod plugin {
    pub mod game;
    pub mod main_menu;
    pub mod paused;
    pub mod background;
    pub mod level_manager;
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

        .add_state::<states::GameState>()

        .add_state::<states::GameInitState>()

        .add_systems(Startup, setup)

        .add_plugins((
            plugin::level_manager::LevelPlugin,
            plugin::main_menu::MenuPlugin,
            plugin::game::GamePlugin,
            plugin::background::BackgroundPlugin,
            plugin::paused::PausedMenuPlugin,
            )
        ).run();
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

