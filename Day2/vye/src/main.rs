use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("./input.txt");
    let file = BufReader::new(File::open(&path).unwrap());
    let mut score: i32 = 0;
    for wrapped_line in file.lines() {
        let line = wrapped_line.unwrap(); // may panic, should not for valid data
        match line.as_str() { // (A=1) <- (B=2) <- (C=3) <- (A=1), (X=loss) = 0, (Y=draw) = 3, (Z=win) = 6
            "A X" => score += 3,
            "B X" => score += 1,
            "C X" => score += 2,
            "A Y" => score += 4,
            "B Y" => score += 5,
            "C Y" => score += 6,
            "A Z" => score += 8,
            "B Z" => score += 9,
            "C Z" => score += 7,
            _ => ()
        }
    }
    print!("{}\n", score);
}