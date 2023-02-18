use crate::app::style::Theme;
use iced::widget::rule;

#[derive(Default)]
pub enum Rule {
    #[default]
    Default,
}

impl rule::StyleSheet for Theme {
    type Style = Rule;

    fn appearance(&self, style: &Self::Style) -> rule::Appearance {
        match style {
            Rule::Default => rule::Appearance {
                color: self.outline,
                width: 1,
                radius: 0.0,
                fill_mode: rule::FillMode::Full,
            },
        }
    }
}
