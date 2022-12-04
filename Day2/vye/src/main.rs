use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("./input.txt");
    let file = BufReader::new(File::open(&path).unwrap());
    let mut score0: i32 = 0;
    let mut score1: i32 = 0;
    for wrapped_line in file.lines() {
        let line = wrapped_line.unwrap(); // may panic, should not for valid data
        match line.as_str() { // (loss) = 0, (draw) = 3, (win) = 6
            "A X" => { score0 += 4; score1 += 3; },
            "B X" => { score0 += 1; score1 += 1; },
            "C X" => { score0 += 7; score1 += 2; },
            "A Y" => { score0 += 8; score1 += 4; },
            "B Y" => { score0 += 5; score1 += 5; },
            "C Y" => { score0 += 2; score1 += 6; },
            "A Z" => { score0 += 3; score1 += 8; },
            "B Z" => { score0 += 9; score1 += 9; },
            "C Z" => { score0 += 6; score1 += 7; },
            _ => ()
        }
    }
    print!("Part 1: {}\n", score0);
    print!("Part 2: {}\n", score1);
}