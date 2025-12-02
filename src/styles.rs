use iced::{Background, Border, Color, Shadow, Theme, Vector};
use iced::widget::{button, container};

pub fn terminal_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: None,
        border: Border {
            color: Color::from_rgb(0.7, 0.7, 0.7),
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SecondaryButtonStyle;

impl button::StyleSheet for SecondaryButtonStyle {
    type Style = Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.85, 0.85, 0.85))),
            text_color: Color::from_rgb(0.2, 0.2, 0.2),
            border: Border {
                color: Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 2.0.into(),
            },
            shadow: Shadow::default(),
            shadow_offset: Vector::default(),
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.75, 0.75, 0.75))),
            text_color: Color::from_rgb(0.2, 0.2, 0.2),
            border: Border {
                color: Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 2.0.into(),
            },
            shadow: Shadow::default(),
            shadow_offset: Vector::default(),
        }
    }

    fn disabled(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
            text_color: Color::from_rgb(0.5, 0.5, 0.5),
            border: Border {
                color: Color::from_rgb(0.8, 0.8, 0.8),
                width: 1.0,
                radius: 2.0.into(),
            },
            shadow: Shadow::default(),
            shadow_offset: Vector::default(),
        }
    }
}
