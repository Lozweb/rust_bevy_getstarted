use bevy::prelude::*;
use crate::states::menu_element;
use crate::states::menu_element::button_system;
use crate::states::states::{despawn_screen, GameInitState, GameState};
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuPausedState {
    Main,
    Settings,
    #[default]
    Disabled,
}

#[derive(Component)]
struct OnPausedMainMenuScreen;
#[derive(Component)]
struct OnPausedSettingsMenuScreen;


#[derive(Component)]
pub enum PausedMenuAction {
    Play,
    Settings,
    BackToMainMenu,
    Quit
}

pub struct PausedMenuPlugin;
impl Plugin for PausedMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<MenuPausedState>()

            .add_systems(OnEnter(GameState::Paused), paused_setup)

            .add_systems(OnEnter(MenuPausedState::Main), paused_menu_setup)
            .add_systems(OnExit(MenuPausedState::Main), despawn_screen::<OnPausedMainMenuScreen>)

            .add_systems(OnEnter(MenuPausedState::Settings), paused_menu_settings_setup)
            .add_systems(OnExit(MenuPausedState::Settings), despawn_screen::<OnPausedSettingsMenuScreen>)

            .add_systems(Update, (menu_action, button_system).run_if(in_state(GameState::Paused)));
    }
}

fn paused_setup(mut menu_state: ResMut<NextState<MenuPausedState>>) {
    menu_state.set(MenuPausedState::Main)
}

fn paused_menu_setup(mut commands: Commands){
    commands.spawn((
        menu_element::menu_main_container_bundle(), OnPausedMainMenuScreen,
    ))

        .with_children(|parent|{

            parent.spawn(menu_element::menu_container_bundle())

                .with_children(|parent| {

                    parent.spawn(menu_element::menu_title("Paused"));

                    parent.spawn(menu_element::menu_button_paused_bundle(PausedMenuAction::Play))
                        .with_children(|parent| {
                            parent.spawn(menu_element::menu_button_text("Resumed"));
                        });

                    parent.spawn(menu_element::menu_button_paused_bundle(PausedMenuAction::Settings))
                        .with_children(|parent| {
                            parent.spawn(menu_element::menu_button_text("Settings"));
                        });

                    parent.spawn(menu_element::menu_button_paused_bundle(PausedMenuAction::Quit))
                        .with_children(|parent| {
                            parent.spawn(menu_element::menu_button_text("Quit"));
                        });

                });
        });
}

fn paused_menu_settings_setup(mut commands: Commands){
    commands.spawn((
        menu_element::menu_main_container_bundle(), OnPausedSettingsMenuScreen,
    ))

        .with_children(|parent|{

            parent.spawn(menu_element::menu_container_bundle())

                .with_children(|parent| {

                    parent.spawn(menu_element::menu_title("Settings"));

                    parent.spawn(menu_element::menu_button_paused_bundle(PausedMenuAction::BackToMainMenu))
                        .with_children(|parent| {
                            parent.spawn(menu_element::menu_button_text("Back"));
                        });

                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &PausedMenuAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<NextState<MenuPausedState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_init_state: ResMut<NextState<GameInitState>>
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                PausedMenuAction::Play => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuPausedState::Disabled);
                },
                PausedMenuAction::Quit => {
                    game_state.set(GameState::Menu);
                    game_init_state.set(GameInitState::No);
                    menu_state.set(MenuPausedState::Disabled);
                },
                PausedMenuAction::Settings => {
                    menu_state.set(MenuPausedState::Settings);
                },
                PausedMenuAction::BackToMainMenu => {
                    menu_state.set(MenuPausedState::Main);
                }
            }
        }
    }
}