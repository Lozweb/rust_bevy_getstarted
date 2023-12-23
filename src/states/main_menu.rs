use bevy::app::AppExit;
use bevy::prelude::*;
use crate::states::screen::{ResolutionMode, set_current_screen_resolution};
use crate::states::game::OnGameScreen;
use crate::states::menu_element;
use crate::states::menu_element::button_system;
use crate::states::states::{despawn_screen, GameInitState, GameState};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Settings,
    Resolution,
    #[default]
    Disabled,
}
#[derive(Component)]
struct OnMainMenuScreen;
#[derive(Component)]
struct OnSettingsMenuScreen;
#[derive(Component)]
struct OnResolutionMenuScreen;
#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Settings,
    Resolution,
    High,
    Medium,
    Low,
    BackToMainMenu,
    BackToSettings,
    Quit
}
pub struct  MenuPlugin;

// TODO navigation keyboard

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<MenuState>()

            .add_systems(OnEnter(GameState::Menu), (menu_setup, despawn_screen::<OnGameScreen>))

            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)

            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
            .add_systems(OnExit(MenuState::Settings), despawn_screen::<OnSettingsMenuScreen>)

            .add_systems(OnEnter(MenuState::Resolution), resolution_menu_setup)
            .add_systems(OnExit(MenuState::Resolution), despawn_screen::<OnResolutionMenuScreen>)

            .add_systems(Update, (menu_action, button_system).run_if(in_state(GameState::Menu)));
    }
}
fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}
fn main_menu_setup(mut commands: Commands){

    commands.spawn((
        menu_element::menu_main_container_bundle(), OnMainMenuScreen,
    ))

        .with_children(|parent|{

            parent.spawn(menu_element::menu_container_bundle())

                .with_children(|parent| {

                    parent.spawn(menu_element::menu_title("Space Shooter"));

                    parent.spawn(menu_element::menu_button_bundle(MenuButtonAction::Play))
                        .with_children(|parent| {
                            parent.spawn(menu_element::menu_button_text("New Game"));
                        });

                    parent.spawn(menu_element::menu_button_bundle(MenuButtonAction::Settings))
                        .with_children(|parent| {
                            parent.spawn(menu_element::menu_button_text("Settings"));
                        });

                    parent.spawn(menu_element::menu_button_bundle(MenuButtonAction::Quit))
                        .with_children(|parent| {
                            parent.spawn(menu_element::menu_button_text("Quit"));
                        });

                });
        });
}
fn settings_menu_setup(mut commands: Commands) {
    commands.spawn((
        menu_element::menu_main_container_bundle(), OnMainMenuScreen,
    ))

        .with_children(|parent|{

            parent.spawn(menu_element::menu_container_bundle())

                .with_children(|parent| {

                    parent.spawn(menu_element::menu_title("Settings"));

                    parent.spawn(menu_element::menu_button_bundle(MenuButtonAction::Resolution))
                        .with_children(|parent| {
                            parent.spawn(menu_element::menu_button_text("Resolution"));
                        });

                    parent.spawn(menu_element::menu_button_bundle(MenuButtonAction::BackToMainMenu))
                        .with_children(|parent| {
                            parent.spawn(menu_element::menu_button_text("Back"));
                        });

                });
        });
}
fn resolution_menu_setup(mut commands: Commands){
    commands.spawn((
        menu_element::menu_main_container_bundle(), OnResolutionMenuScreen,
    ))

        .with_children(|parent|{

            parent.spawn(menu_element::menu_container_bundle())

                .with_children(|parent| {

                    parent.spawn(menu_element::menu_title("Resolution"));

                    parent.spawn(menu_element::menu_button_bundle(MenuButtonAction::High))
                        .with_children(|parent| {
                            parent.spawn(menu_element::menu_button_text("High:1920x1080"));
                        });

                    parent.spawn(menu_element::menu_button_bundle(MenuButtonAction::Medium))
                        .with_children(|parent| {
                            parent.spawn(menu_element::menu_button_text("Medium:1600x900"));
                        });

                    parent.spawn(menu_element::menu_button_bundle(MenuButtonAction::Low))
                        .with_children(|parent| {
                            parent.spawn(menu_element::menu_button_text("Low:1280x720"));
                        });

                    parent.spawn(menu_element::menu_button_bundle(MenuButtonAction::BackToSettings))
                        .with_children(|parent| {
                            parent.spawn(menu_element::menu_button_text("Back"));
                        });

                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_init_sate: ResMut<NextState<GameInitState>>,
    mut windows: Query<&mut Window>
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit)
                },

                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                    game_init_sate.set(GameInitState::Starting);
                    menu_state.set(MenuState::Disabled);
                },

                MenuButtonAction::Settings => {
                    menu_state.set(MenuState::Settings)
                },

                MenuButtonAction::Resolution => {
                    menu_state.set(MenuState::Resolution)
                },

                MenuButtonAction::High => unsafe {
                    let screen = ResolutionMode::High.get_resolution();
                    let mut window = windows.single_mut();
                    window.resolution.set(screen.width, screen.height);
                    set_current_screen_resolution(ResolutionMode::High);
                },

                MenuButtonAction::Medium => unsafe {
                    let screen = ResolutionMode::Medium.get_resolution();
                    let mut window = windows.single_mut();
                    window.resolution.set(screen.width, screen.height);
                    set_current_screen_resolution(ResolutionMode::Medium);
                },

                MenuButtonAction::Low => unsafe {
                    let screen = ResolutionMode::Low.get_resolution();
                    let mut window = windows.single_mut();
                    window.resolution.set(screen.width, screen.height);
                    set_current_screen_resolution(ResolutionMode::Low);
                },

                MenuButtonAction::BackToMainMenu => {
                    menu_state.set(MenuState::Main)
                },
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
            }
        }
    }
}



