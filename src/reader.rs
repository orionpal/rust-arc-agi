use rand::seq::{IndexedRandom};
use std::fs;
use std::path::Path;
use serde_json::{from_str, Value};
use crate::player::{Player};
use crate::puzzle::{Grid, Pair, Puzzle};
use crate::reader;

pub fn get_random_file_from_directory<P: AsRef<Path>>(dir: P) -> Option<String> {
    let entries = fs::read_dir(dir).ok()?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .collect::<Vec<_>>();

    let mut rng = rand::thread_rng();
    entries.choose(&mut rng).map(|e| e.path().to_string_lossy().into_owned())
}

pub fn load_puzzle(file_path: &str) -> Puzzle {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    let json: Value = from_str(&data).expect("JSON was not well-formatted");

    // Print the JSON data to inspect it
    // println!("{:#?}", json);

    // Assuming the JSON structure follows the provided example
    let mut train = Vec::new();
    if let Some(train_data) = json.get("train").and_then(|t| t.as_array()) {
        for pair in train_data {
            if let (Some(input), Some(output)) = (pair.get("input"), pair.get("output")) {
                let input_grid = create_grid_from_value(input);
                let output_grid = create_grid_from_value(output);
                train.push(Pair { input: input_grid, output: output_grid });
            }
        }
    }

    let mut test = Pair {
        input: Grid::new(0,0),
        output: Grid::new(0,0)
    };
    if let Some(test_data) = json.get("test").and_then(|t| t.as_array()) {
        for test_case in test_data {
            if let (Some(input), Some(output)) = (test_case.get("input"), test_case.get("output")) {
                let input_grid = create_grid_from_value(input);
                let output_grid = create_grid_from_value(output);
                test = Pair {
                    input: input_grid,
                    output: output_grid
                }
            }
        }
    }

    Puzzle { train, test }
}


fn create_grid_from_value(value: &Value) -> Grid {
    if let Some(array) = value.as_array() {
        let height = array.len();
        let width = array.first().map_or(0, |row| row.as_array().map_or(0, |r| r.len()));
        let mut grid = Grid::new(width, height);
        for (y, row) in array.iter().enumerate() {
            if let Some(row_array) = row.as_array() {
                for (x, cell) in row_array.iter().enumerate() {
                    if let Some(value) = cell.as_u64() {
                        grid.set(x, y, value as u8);
                    }
                }
            }
        }
        grid
    } else {
        Grid::new(0, 0)
    }
}

pub fn try_reading_random_puzzle(filepath: String, mut player: &mut Box<dyn Player>) {
    match get_random_file_from_directory(filepath) {
        Some(file_path) => {
            println!("Random file: {}", file_path);
            let puzzle = load_puzzle(&file_path);
            for train in &puzzle.train {
                println!("input:");
                println!("{}", train.input);
                println!("output:");
                println!("{}", train.output);
            }
            println!("final test:");
            println!("{}", puzzle.test.input);
            println!("attempted solution:");
            println!("{}", player.solve(&puzzle));
            println!("actual solution:");
            println!("{}", puzzle.test.output);
        },
        None => println!("No files found in the directory or unable to read the directory."),
    }
}