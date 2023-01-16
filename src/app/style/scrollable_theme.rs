use super::theme::Theme;
use iced::widget::scrollable;
use iced::Color;

/// The style of a scrollable.
#[derive(Default)]
pub enum Scrollable {
    #[default]
    Default,
}

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, style: &Self::Style) -> scrollable::Scrollbar {
        match style {
            Scrollable::Default => {
                let palette = self;

                scrollable::Scrollbar {
                    background: palette.background.into(),
                    border_radius: 2.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    scroller: scrollable::Scroller {
                        color: palette.background,
                        border_radius: 2.0,
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                }
            }
        }
    }

    fn hovered(&self, style: &Self::Style) -> scrollable::Scrollbar {
        match style {
            Scrollable::Default => {
                let palette = self;

                scrollable::Scrollbar {
                    background: palette.background.into(),
                    border_radius: 2.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    scroller: scrollable::Scroller {
                        color: palette.text,
                        border_radius: 2.0,
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                    },
                }
            }
        }
    }

    fn dragging(&self, style: &Self::Style) -> scrollable::Scrollbar {
        match style {
            Scrollable::Default => self.hovered(style),
        }
    }
}
