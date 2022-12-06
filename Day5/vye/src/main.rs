use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use regex::Regex;

fn main() {
    let path = Path::new("./input.txt");
    let file = BufReader::new(File::open(&path).unwrap());
    let nbr = Regex::new("\\d+").unwrap();
    let cnk = Regex::new(".{3,4}").unwrap();
    let mut stack9000_1: Vec<char> = vec![];
    let mut stack9000_2: Vec<char> = vec![];
    let mut stack9000_3: Vec<char> = vec![];
    let mut stack9000_4: Vec<char> = vec![];
    let mut stack9000_5: Vec<char> = vec![];
    let mut stack9000_6: Vec<char> = vec![];
    let mut stack9000_7: Vec<char> = vec![];
    let mut stack9000_8: Vec<char> = vec![];
    let mut stack9000_9: Vec<char> = vec![];
    macro_rules! stack9000{
            [$i:expr]=>{
                match ($i) {
                    1 => &mut stack9000_1,
                    2 => &mut stack9000_2,
                    3 => &mut stack9000_3,
                    4 => &mut stack9000_4,
                    5 => &mut stack9000_5,
                    6 => &mut stack9000_6,
                    7 => &mut stack9000_7,
                    8 => &mut stack9000_8,
                    9 => &mut stack9000_9,
                    _ => panic!("stack number not defined")
                }
            }
        }
    let mut stack9001_1: Vec<char> = vec![];
    let mut stack9001_2: Vec<char> = vec![];
    let mut stack9001_3: Vec<char> = vec![];
    let mut stack9001_4: Vec<char> = vec![];
    let mut stack9001_5: Vec<char> = vec![];
    let mut stack9001_6: Vec<char> = vec![];
    let mut stack9001_7: Vec<char> = vec![];
    let mut stack9001_8: Vec<char> = vec![];
    let mut stack9001_9: Vec<char> = vec![];
    macro_rules! stack9001{
            [$i:expr]=>{
                match ($i) {
                    1 => &mut stack9001_1,
                    2 => &mut stack9001_2,
                    3 => &mut stack9001_3,
                    4 => &mut stack9001_4,
                    5 => &mut stack9001_5,
                    6 => &mut stack9001_6,
                    7 => &mut stack9001_7,
                    8 => &mut stack9001_8,
                    9 => &mut stack9001_9,
                    _ => panic!("stack number not defined")
                }
            }
        }
    let mut lines = file.lines();
    'build_port: loop {
        let wrapped_line = lines.next().unwrap();
        let line = wrapped_line.unwrap(); // may panic, should not for valid data
        let mut chunk_id: u8 = 0;
        for chunk in cnk.find_iter(&line).map(|m| m.as_str().chars().nth(1).unwrap()) {
            chunk_id += 1;
            match chunk {
                ' ' => (),
                'A'..='Z' => { stack9000![chunk_id].insert(0, chunk); stack9001![chunk_id].insert(0, chunk); },
                '0'..='9' => break 'build_port,
                _ => panic!("not reachable with expected input")
            }
        }
    }
    assert_eq!(0, lines.next().unwrap().unwrap().len()); // gap between port diagram and instructions
    for wrapped_line in lines {
        let line = wrapped_line.unwrap(); // may panic, should not for valid data
        let mut instructions = nbr.find_iter(&line);
        let count = instructions.next().unwrap().as_str().parse::<u8>().unwrap();
        let from = instructions.next().unwrap().as_str().parse::<u8>().unwrap();
        let to = instructions.next().unwrap().as_str().parse::<u8>().unwrap();
        let level = stack9001![to].len();
        for _ in 0..count {
            let c9000 = stack9000![from].pop().unwrap();
            let c9001 = stack9001![from].pop().unwrap();
            stack9000![to].push(c9000);
            stack9001![to].insert(level, c9001);
        }
    }
    print!("Part 1: {}{}{}{}{}{}{}{}{}\n",
        stack9000_1.last().unwrap(),
        stack9000_2.last().unwrap(),
        stack9000_3.last().unwrap(),
        stack9000_4.last().unwrap(),
        stack9000_5.last().unwrap(),
        stack9000_6.last().unwrap(),
        stack9000_7.last().unwrap(),
        stack9000_8.last().unwrap(),
        stack9000_9.last().unwrap()
    );
    print!("Part 2: {}{}{}{}{}{}{}{}{}\n",
        stack9001_1.last().unwrap(),
        stack9001_2.last().unwrap(),
        stack9001_3.last().unwrap(),
        stack9001_4.last().unwrap(),
        stack9001_5.last().unwrap(),
        stack9001_6.last().unwrap(),
        stack9001_7.last().unwrap(),
        stack9001_8.last().unwrap(),
        stack9001_9.last().unwrap()
    );
}
