use iced::Color;

macro_rules! color {
    ($red:expr, $green:expr, $blue:expr) => {
        Color::from_rgba(
            $red as f32 / 255.0,
            $green as f32 / 255.0,
            $blue as f32 / 255.0,
            1.0,
        )
    };
    ($red:expr, $green:expr, $blue:expr, $opacity:expr) => {
        Color::from_rgba(
            $red as f32 / 255.0,
            $green as f32 / 255.0,
            $blue as f32 / 255.0,
            $opacity as f32 / 255.0,
        )
    };
}

pub struct Theme {
    pub text: Color,
    pub background: Color,
    pub button_no_hover: Color,
    pub button_hover: Color,
    pub border_color: Color,
    pub input_background: Color,
}

impl Theme {
    pub const NORMAL: Self = Self {
        text: Color::WHITE,
        background: color!(13, 16, 23),
        input_background: color!(20, 23, 26),
        button_no_hover: color!(17, 21, 28),
        button_hover: color!(71, 82, 102),
        border_color: color!(19, 23, 33),
    };
}

impl Default for Theme {
    fn default() -> Self {
        Self::NORMAL
    }
}
