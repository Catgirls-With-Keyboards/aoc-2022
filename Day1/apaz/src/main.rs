fn part1(input: &String) -> i32 {
  let mut largest : i32 = 0;
  for s in input.split("\n\n") {
    let mut sum : i32 = 0;
    for l in s.split("\n") {
      if l.is_empty() { continue };
      sum += l.parse::<i32>().expect(&l);
    }
    if sum > largest { largest = sum }
  }
  largest
}

fn part2(input: &String) -> i32 {
  let mut l1 : i32 = 0;
  let mut l2 : i32 = 0;
  let mut l3 : i32 = 0;
  for s in input.split("\n\n") {
    let mut sum : i32 = 0;
    for l in s.split("\n") {
      if l.is_empty() { continue };
      sum += l.parse::<i32>().expect(&l);
    }
    // I don't need heaps or priority queues.
    if      sum > l1 { l3 = l2; l2 = l1; l1 = sum; }
    else if sum > l2 { l3 = l2; l2 = sum; }
    else if sum > l3 { l3 = sum; }
  }
  l1 + l2 + l3
}

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  let p1 = part1(&input);
  println!("{p1}");
  let p2 = part2(&input);
  println!("{p2}");
}
