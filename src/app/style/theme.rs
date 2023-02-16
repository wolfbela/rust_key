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
    pub main: Color,
    pub secondary: Color,
    pub outline: Color,
}

impl Theme {
    pub const NORMAL: Self = Self {
        main: color!(255, 255, 255),
        secondary: color!(203, 243, 240),
        outline: color!(46, 196, 182),
    };
}

impl Default for Theme {
    fn default() -> Self {
        Self::NORMAL
    }
}
