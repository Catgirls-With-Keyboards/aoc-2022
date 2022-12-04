use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("./input.txt");
    let file = BufReader::new(File::open(&path).unwrap());
    let mut max0: i32 = 0;
    let mut max1: i32 = 0;
    let mut max2: i32 = 0;
    let mut total: i32 = 0;
    for wrapped_line in file.lines() {
        let line = wrapped_line.unwrap(); // may panic, should not for valid data
        if line.is_empty() {
            if total > max0 { max0 = total; }
            if total > max1 { max0 = max1; max1 = total; }
            if total > max2 { max1 = max2; max2 = total; }
            total = 0;
        } else {
            total += line.parse::<i32>().unwrap(); // may panic, should not for valid data
        }
    }
    print!("Part 1: {}\n", max2);
    print!("Part 2: {}\n", max0 + max1 + max2);
}
