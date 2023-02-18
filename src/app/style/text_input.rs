use super::theme::Theme;
use iced::widget::text_input;
use iced::Color;

/// The style of a text input.
#[derive(Default)]
pub enum TextInput {
    /// The default style.
    #[default]
    Default,
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Color::TRANSPARENT.into(),
            border_radius: 5.0,
            border_width: 3.0,
            border_color: self.secondary,
        }
    }

    fn hovered(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Color::TRANSPARENT.into(),
            border_radius: 5.0,
            border_width: 3.0,
            border_color: Color {
                a: 0.9,
                ..self.outline
            },
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Color::TRANSPARENT.into(),
            border_radius: 5.0,
            border_width: 3.0,
            border_color: self.outline,
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        self.outline
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        self.outline
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        self.main
    }
}
