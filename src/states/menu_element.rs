use bevy::prelude::{AlignItems, BackgroundColor, Button, ButtonBundle, Changed, Color, Component, default, FlexDirection, Interaction, JustifyContent, NodeBundle, Query, Style, TextBundle, UiRect, Val, With};
use bevy::text::TextStyle;
use crate::states::main_menu::MenuButtonAction;
use crate::states::paused::PausedMenuAction;

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn menu_button_style() -> Style {
    Style{
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
pub fn menu_button_text_style() -> TextStyle {
    TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    }
}

pub fn menu_main_container_bundle() -> NodeBundle {
    NodeBundle{
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    }
}
pub fn menu_container_bundle() -> NodeBundle {
    NodeBundle{
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::BLACK.into(),
        ..default()
    }
}

pub fn menu_title(title: &str) -> TextBundle{
    TextBundle::from_section(
        title,
        TextStyle{
            font_size: 80.0,
            color: TEXT_COLOR,
            ..default()
        }
    ).with_style(Style {
        margin: UiRect::all(Val::Px(50.0)),
        ..default()
    })
}

pub fn menu_button_bundle(action: MenuButtonAction) -> (ButtonBundle, MenuButtonAction) {
    (ButtonBundle  {
        style: menu_button_style(),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    },
     action
    )
}

pub fn menu_button_paused_bundle(action: PausedMenuAction) -> (ButtonBundle, PausedMenuAction) {
    (ButtonBundle  {
        style: menu_button_style(),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    },
     action
    )
}

pub fn menu_button_text(text: &str)-> TextBundle{
    TextBundle::from_section(
        text,
        menu_button_text_style(),
    )
}

#[derive(Component)]
pub struct SelectedOption;

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>)
    >
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}