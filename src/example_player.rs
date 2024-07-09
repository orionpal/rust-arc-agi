use rand::Rng;
use crate::player::{Player};
use crate::puzzle::{Grid, Pair, Puzzle};

#[derive(Debug)]
pub struct MyPlayer {

}

impl Player for MyPlayer {
    fn solve(&mut self, input: &Puzzle) -> Grid {
        // Implement solving logic and return the output grid
        println!("Learning from training data (except not really)");
        let training_stuff = &input.train;
        println!("Solving the test input grid");
        let test_grid = &input.test.input;
        let mut rng = rand::thread_rng();
        let width = rng.gen_range(0..=30);
        let height = rng.gen_range(0..=30);
        let mut answer_grid = Grid::new(width, height);
        for y in 0..height {
            for x in 0..width {
                answer_grid.set(x, y, rng.gen_range(0..=9))
            }
        }
        answer_grid
    }
}
