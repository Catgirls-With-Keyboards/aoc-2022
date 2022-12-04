use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("./input.txt");
    let file = BufReader::new(File::open(&path).unwrap());
    let mut cost: u32 = 0;
    for wrapped_line in file.lines() {
        let line = wrapped_line.unwrap(); // may panic, should not for valid data
        let segment_len = line.len() / 2;
        let mut bytes = line.as_str().bytes(); // raw iterator
        let mut mask: u64 = 0;
        for _ in 0..segment_len {
            let byte = bytes.next().unwrap(); // may panic, should not for valid data
            let score = if byte > 96 {byte - 96} else {byte - 38};
            mask |= 1 << (score as u64);
        }
        'find_overlap: for _ in 0..segment_len {
            let byte = bytes.next().unwrap(); // may panic, should not for valid data
            let score = if byte > 96 {byte - 96} else {byte - 38};
            if mask & (1 << (score as u64)) != 0 {
                cost += score as u32;
                break 'find_overlap;
            }
        }
    }
    print!("Part 1: {}\n", cost);
}
