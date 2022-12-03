fn part1(input: &String) -> i32 {
  let mut score = 0;
  for l in input.split("\n") {
    if l == "" {continue}
    let mut spt = l.split(" ").peekable();
    let o = spt.next().unwrap().chars().next().unwrap() as i32 - ('A' as i32) + 1;
    let m = spt.next().unwrap().chars().next().unwrap() as i32 - ('X' as i32) + 1;
    let mut win = false;
    if (m==1 && o==3) || (m==2 && o==1) || (m==3 && o==2) {
      win = true;
    }
    score += if win {6} else {if m==o {3} else {0}};
    score += m;
  }
  score
}

fn _score(o: i32, m: i32) -> i32 {
  let win = (m==1 && o==3) || (m==2 && o==1) || (m==3 && o==2);
  (if win {6} else {if m==o {3} else {0}} + m)
}

fn part2(input: &String) -> i32 {
  let mut score: i32 = 0;
  for l in input.split("\n") {
    if l == "" {continue}
    let mut spt = l.split(" ").peekable();
    let o = spt.next().unwrap().chars().next().unwrap() as i32 - ('A' as i32) + 1;
    let m = spt.next().unwrap().chars().next().unwrap() as i32 - ('X' as i32) + 1;
    let lose = if o==1 {3} else {if o==2 {1} else {2}};
    let tie  = o;
    let win  = if o==3 {1} else {if o==1 {2} else {3}};
    score += _score(o, if m==1 {lose} else {if m==2 {tie} else {win}});
  }
  score
}

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  let p1 = part1(&input);
  println!("{p1}");
  let p2 = part2(&input);
  println!("{p2}");
}
