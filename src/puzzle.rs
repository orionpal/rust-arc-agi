use serde::{Deserialize, Serialize};
use std::fmt;
/// Defines an instance of one arc puzzle, which should consist of:
/// Json file with 2 fields:
/// 1. train (list of training grid data pairs)
/// 2. test (also grid data pairs, input/output)

#[derive(Serialize, Deserialize, Debug)]
pub struct Puzzle {
    pub train: Vec<Pair>,
    pub test: Pair
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pair {
    pub input: Grid,
    pub output: Grid
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Grid {
    /// data is a list of integer values between 0 and 9 inclusive
    /// Largest dimensions are 30x30
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<u8>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width * height <= 30 * 30, "Grid size exceeds 30x30 limit");
        Self {
            width,
            height,
            data: vec![vec![0; width]; height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.data[y][x] = value;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let value = self.get(x, y);
                let color = color_for_value(value);
                write!(f, "{}{:X} \x1b[0m", color, value)?;  // Reset color after each value
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn color_for_value(value: u8) -> &'static str {
    match value {
        0 => "\x1b[0;30m",  // Black
        1 => "\x1b[0;31m",  // Red
        2 => "\x1b[0;32m",  // Green
        3 => "\x1b[0;33m",  // Yellow
        4 => "\x1b[0;34m",  // Blue
        5 => "\x1b[0;35m",  // Magenta
        6 => "\x1b[0;36m",  // Cyan
        7 => "\x1b[0;37m",  // Gray
        8 => "\x1b[0;38m",  // White
        9 => "\x1b[0;40m",  // ?
        _ => "\x1b[0m",     // Reset
    }
}
