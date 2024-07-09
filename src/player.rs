use crate::puzzle::{Grid, Pair, Puzzle};
use std::fmt::Debug;

/// Template for something that learns and then attempts

pub trait Player: Debug {
    fn solve(&mut self, input: &Puzzle) -> Grid;

}