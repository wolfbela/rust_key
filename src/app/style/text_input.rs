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

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: self.input_background.into(),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: self.border_color,
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: self.input_background.into(),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: self.border_color,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: self.input_background.into(),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: self.border_color,
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        self.background
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        self.background
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        self.background
    }
}
