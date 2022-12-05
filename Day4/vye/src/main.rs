use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use regex::Regex;
use std::cmp::{min, max};

fn main() {
    let path = Path::new("./input.txt");
    let file = BufReader::new(File::open(&path).unwrap());
    let nbr: Regex = Regex::new("\\d+").unwrap();
    let mut subset_count: u32 = 0; // non strict subset
    let mut overlap_count: u32 = 0;
    for wrapped_line in file.lines() {
        let line = wrapped_line.unwrap(); // may panic, should not for valid data
        let mut xs = nbr.find_iter(&line);
        let x0 = xs.next().unwrap().as_str().parse::<u32>().unwrap();
        let x1 = xs.next().unwrap().as_str().parse::<u32>().unwrap();
        let x2 = xs.next().unwrap().as_str().parse::<u32>().unwrap();
        let x3 = xs.next().unwrap().as_str().parse::<u32>().unwrap();
        let e0 = x1 - x0 + 1;
        let e1 = x3 - x2 + 1;
        let total = max(x1, x3) - min(x0, x2) + 1;
        if total == max(e0, e1) { subset_count += 1; }
        if total < e0 + e1 { overlap_count += 1; }

    }
    print!("Part 1: {}\n", subset_count);
    print!("Part 2: {}\n", overlap_count);
}
