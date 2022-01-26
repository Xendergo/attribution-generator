// https://github.com/Xendergo/dnd-stuff/blob/master/server/src/styling.rs

use iced::{widget::*, Background, Color, Vector};

const BORDER_RADIUS: f32 = 12.0;
pub const PADDING: u16 = 12;

pub struct Button();

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector { x: 0.0, y: 0.0 },
            background: None,
            border_color: Color::WHITE,
            border_radius: BORDER_RADIUS,
            border_width: 2.0,
            text_color: Color::WHITE,
        }
    }
}

pub struct TextInput();

impl text_input::StyleSheet for TextInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(Color::TRANSPARENT),
            border_radius: BORDER_RADIUS,
            border_width: 2.0,
            border_color: Color::WHITE,
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(Color::TRANSPARENT),
            border_radius: BORDER_RADIUS,
            border_width: 2.0,
            border_color: Color::WHITE,
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb8(128, 128, 128)
    }

    fn value_color(&self) -> Color {
        Color::WHITE
    }

    fn selection_color(&self) -> Color {
        Color::from_rgb8(64, 128, 255)
    }
}
