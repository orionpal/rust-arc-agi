use std::env;
use crate::evaluator::run_over_all_files;
use crate::example_player::{MyPlayer};
use crate::player::{Player};
use crate::reader::try_reading_random_puzzle;
use crate::graphics::window::start_app;
mod reader;
mod puzzle;
mod player;
mod example_player;
mod evaluator;
mod graphics;


fn main() {
    let args: Vec<String> = env::args().collect();
    let training_path = "training/";
    let evaluation_path = "evaluation/";
    // put your model here:
    let mut player: Box<dyn Player> = Box::new(MyPlayer { });

    match args.len() {
        1 => {
            // No arguments, try a random file
            start_app();
            try_reading_random_puzzle(training_path.to_string(), &mut player);

        },
        2 => {
            // One argument provided
            let argument = &args[1];
            if argument == "full" {
                // Run over all training and evaluation files
                run_over_all_files(training_path, &mut player);
                run_over_all_files(evaluation_path, &mut player);
            }
            // } else if check_file_exists(training_path, argument) {
            //     // Try solving the specified file in training
            //     player::solve_puzzle(format!("{}/{}", training_path, argument));
            // } else if check_file_exists(evaluation_path, argument) {
            //     // Try solving the specified file in evaluation
            //     player::solve_puzzle(format!("{}/{}", evaluation_path, argument));
            // } else {
            //     eprintln!("File not found in training or evaluation directories.");
            // }
        },
        _ => {
            eprintln!("Invalid number of arguments.");
        }
    }

    // Standard Run
    // 1. Train off of training data set
    // 2. Attempt evaluation set
    // 3. Display results
}
