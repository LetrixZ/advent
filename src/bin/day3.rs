use std::fs;

#[derive(Debug)]
struct Number {
  value: i32,
  start: usize,
  end: usize,
  pos: usize,
}

#[derive(Debug)]
struct Symbol {
  index: usize,
  pos: usize,
}

impl Number {
  fn check_adjacent(&self, symbols: &Vec<Symbol>) -> bool {
    symbols
      .into_iter()
      .filter(|symbol| {
        symbol.pos >= self.pos.saturating_sub(1) && symbol.pos <= self.pos.saturating_add(1)
      })
      .find(|symbol| {
        symbol.index >= self.start.saturating_sub(1) && symbol.index <= self.end.saturating_add(1)
      })
      .is_some()
  }
}

impl Symbol {
  fn check_power(&self, numbers: &Vec<Number>) -> i32 {
    let numbers: Vec<&Number> = numbers
      .into_iter()
      .filter(|number| {
        number.pos >= self.pos.saturating_sub(1)
          && number.pos <= self.pos.saturating_add(1)
          && self.index >= number.start.saturating_sub(1)
          && self.index <= number.end.saturating_add(1)
      })
      .collect();

    if numbers.len() == 2 {
      numbers
        .into_iter()
        .map(|number| number.value)
        .reduce(|acc, number| acc * number)
        .unwrap()
    } else {
      0
    }
  }
}

fn main() {
  let content = fs::read_to_string("./assets/day3.txt").unwrap();

  part_one(&content);
  part_two(&content);
}

fn part_one(content: &String) {
  let mut symbols: Vec<Symbol> = vec![];
  let mut numbers: Vec<Number> = vec![];

  for (pos, line) in content.lines().into_iter().enumerate() {
    let mut n = String::new();

    for (i, char) in line.chars().into_iter().enumerate() {
      if char.is_numeric() {
        n.push(char)
      } else {
        if char != '.' {
          symbols.push(Symbol { index: i, pos });
        }

        if !n.is_empty() {
          numbers.push(Number {
            value: n.trim().parse().unwrap(),
            start: i - n.trim().len(),
            pos,
            end: i - 1,
          });
        }

        n = String::new();
      }
    }
  }

  let result = numbers
    .into_iter()
    .filter(|n| n.check_adjacent(&symbols))
    .map(|n| n.value)
    .reduce(|acc, n| acc + n)
    .unwrap();

  println!("Part One: {result}")
}

fn part_two(content: &String) {
  let mut symbols: Vec<Symbol> = vec![];
  let mut numbers: Vec<Number> = vec![];

  for (pos, line) in content.lines().into_iter().enumerate() {
    let mut n = String::new();

    for (i, char) in line.chars().into_iter().enumerate() {
      if char.is_numeric() {
        n.push(char)
      } else {
        if char == '*' {
          symbols.push(Symbol { index: i, pos });
        }

        if !n.is_empty() {
          numbers.push(Number {
            value: n.trim().parse().unwrap(),
            start: i - n.trim().len(),
            pos,
            end: i - 1,
          });
        }

        n = String::new();
      }
    }
  }

  let result = symbols
    .into_iter()
    .map(|s| s.check_power(&numbers))
    .reduce(|acc, s| acc + s)
    .unwrap();

  println!("Part Two: {result}")
}
