use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

const FULL: u64 = 9007199254740990;

fn main() {
    let path = Path::new("./input.txt");
    let file = BufReader::new(File::open(&path).unwrap());
    let mut line_cost: u32 = 0;
    let mut group_cost: u32 = 0;

    let mut group_mask: u64 = FULL;
    let mut group_index: u8 = 0;

    for wrapped_line in file.lines() {
        let line = wrapped_line.unwrap(); // may panic, should not for valid data

        group_index += 1;
        let segment_len = line.len() / 2;
        let mut bytes = line.as_str().bytes(); // raw iterator
        let mut line_mask: u64 = 0;
        let mut halve_line_mask: u64 = 0;

        for _ in 0..segment_len {

            let byte = bytes.next().unwrap(); // may panic, should not for valid data
            let score = if byte > 96 {byte - 96} else {byte - 38};
            let mask: u64 = 1 << (score as u64);
            line_mask |= mask; // add line set

            halve_line_mask |= mask; // build halve-line set
        }

        'find_overlap: loop {

            let byte = bytes.next().unwrap(); // may panic, should not for valid data
            let score = if byte > 96 {byte - 96} else {byte - 38};
            let mask: u64 = 1 << (score as u64);
            line_mask |= mask; // add line set

            if halve_line_mask & mask != 0 { // read halve-line set
                line_cost += score as u32;
                break 'find_overlap;
            }
        }

        for byte in bytes {
            
            let score = if byte > 96 {byte - 96} else {byte - 38};
            let mask: u64 = 1 << (score as u64);
            line_mask |= mask; // add line set

        }

        group_mask &= line_mask;

        if group_index == 3 { // per group of 3
            group_index = 0;
            group_cost += group_mask.trailing_zeros();
            group_mask = FULL;
        }
    }
    print!("Part 1: {}\n", line_cost);
    print!("Part 2: {}\n", group_cost);
}
