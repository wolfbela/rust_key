pub mod button_theme;
pub mod container;
pub mod rule;
pub mod scrollable_theme;
pub mod text_input;
pub mod theme;

use iced::{application, widget::text, Color};

use theme::Theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum Application {
    #[default]
    Default,
}

impl application::StyleSheet for Theme {
    type Style = Application;

    fn appearance(&self, style: &Self::Style) -> application::Appearance {
        match style {
            Application::Default => application::Appearance {
                background_color: self.main.into(),
                text_color: self.secondary,
            },
        }
    }
}

/*
 * Text
 */
#[derive(Clone, Copy, Default)]
pub enum Text {
    #[default]
    Default,
    Color(Color),
}

impl From<Color> for Text {
    fn from(color: Color) -> Self {
        Text::Color(color)
    }
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::Default => text::Appearance {
                color: Color::BLACK.into(),
            },
            Text::Color(c) => text::Appearance { color: c.into() },
        }
    }
}
