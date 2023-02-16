use super::theme::Theme;
use iced::{widget::container, Color};

/*
 * Container
 */
#[derive(Clone, Copy, Default)]
pub enum Container {
    #[default]
    NotSelectable,
    NotSelected,
    Selected,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Selected => Default::default(),
            Container::NotSelected => todo!(),
            Container::NotSelectable => container::Appearance {
                text_color: None,
                background: None,
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }
}
