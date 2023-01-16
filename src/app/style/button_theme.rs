use super::theme::Theme;
use iced::widget::button;
use iced::Color;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Button {
    #[default]
    Login,
    Actions,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, _style: &Button) -> button::Appearance {
        button::Appearance {
            background: self.button_no_hover.into(),
            border_color: self.border_color,
            text_color: self.text,
            border_radius: 2.0,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            background: self.button_hover.into(),
            ..active
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: iced::Vector::default(),
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: iced::Vector::default(),
            background: active.background.map(|background| match background {
                iced::Background::Color(color) => iced::Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}
