use bevy::prelude::Color;

pub fn cell_color(number: u8) -> Color{
    match number {
        0 => Color::rgb(0.0, 0.0, 0.0),
        1 => Color::rgb(1.0, 0.0, 0.0),
        2 => Color::rgb(0.0, 1.0, 0.0),
        3 => Color::rgb(0.0, 0.0, 1.0),
        4 => Color::rgb(1.0, 1.0, 0.0),
        5 => Color::rgb(0.0, 1.0, 1.0),
        6 => Color::rgb(1.0, 0.0, 1.0),
        7 => Color::rgb(0.5, 0.5, 0.5),
        8 => Color::rgb(0.5, 0.0, 0.0),
        9 => Color::rgb(0.0, 0.5, 0.0),
        _ => panic!("Number out of range!"), // handle unexpected values gracefully
    }
}