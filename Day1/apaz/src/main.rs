use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn open_file(path: &Path) -> String {
  let mut file = File::open(&path).unwrap();
  let mut s = String::new();
  file.read_to_string(&mut s).unwrap();
  s
}

fn main() {
  let input = open_file(Path::new("input.txt"));
  let mut largest : i32 = 0;
  for s in input.split("\n\n") {
    let mut sum : i32 = 0;
    for l in s.split("\n") {
      if l.is_empty() { continue };
      sum += l.parse::<i32>().expect(&l);
    }
    if sum > largest { largest = sum }
  }
  println!("{largest}");
}
