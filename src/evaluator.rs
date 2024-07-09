use std::fs;
use crate::player;
use crate::player::Player;
use crate::reader::load_puzzle;

pub(crate) fn run_over_all_files(dir: &str, mut player: &mut Box<dyn Player>) {
    println!("reading from {}", dir);
    if let Ok(entries) = fs::read_dir(dir) {
        let mut total_count = 0;
        let mut correct_answers = 0;
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    let file_path = entry.path().to_string_lossy().into_owned();
                    let puzzle = load_puzzle(&*file_path);
                    total_count += 1;
                    if player.solve(&puzzle).data == puzzle.test.output.data {
                        correct_answers += 1;
                    }
                }
            }
        }
        println!("Total Score: {}/{}", correct_answers, total_count);
    } else {
        eprintln!("Failed to read directory: {}", dir);
    }
}

