use bevy::prelude::{AlignItems, ButtonBundle, Color, default, FlexDirection, JustifyContent, NodeBundle, Style, TextBundle, UiRect, Val};
use bevy::text::TextStyle;
use crate::states::main_menu::MenuButtonAction;

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

pub fn menu_button_text(text: &str)-> TextBundle{
    TextBundle::from_section(
        text,
        menu_button_text_style(),
    )
}