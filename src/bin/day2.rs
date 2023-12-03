use regex::Regex;
use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Set {
  red: i32,
  green: i32,
  blue: i32,
}

impl Set {
  fn new() -> Self {
    Set {
      red: 0,
      green: 0,
      blue: 0,
    }
  }

  fn is_greater(&self, other: &Set) -> bool {
    self.red >= other.red && self.green >= other.green && self.blue >= other.blue
  }
}

fn main() {
  let content = fs::read_to_string("./assets/day2.txt").expect("Failed to read file");

  part_one(&content);
  part_two(&content);
}

fn part_one(content: &String) {
  let re = Regex::new(r"\d+").unwrap();

  let config_set = Set {
    red: 12,
    green: 13,
    blue: 14,
  };

  let mut possible_games = HashSet::new();

  for line in content.lines() {
    let mut is_possible = true;
    let index = line.find(":").unwrap();
    let game = re
      .captures_iter(line)
      .next()
      .unwrap()
      .get(0)
      .unwrap()
      .as_str()
      .parse::<i32>()
      .unwrap();

    let line = line[index + 1..].to_string();

    for set_str in line.split(";").map(|set| set.trim()) {
      let mut set = Set::new();

      for cube in set_str.split(",") {
        let quantity = re
          .captures_iter(cube)
          .next()
          .unwrap()
          .get(0)
          .unwrap()
          .as_str()
          .parse::<i32>()
          .unwrap();

        if cube.contains("red") {
          set.red = quantity;
        } else if cube.contains("green") {
          set.green = quantity
        } else if cube.contains("blue") {
          set.blue = quantity;
        }
      }

      if !config_set.is_greater(&set) {
        is_possible = false;
        break;
      }
    }

    if is_possible {
      possible_games.insert(game);
    }
  }

  let mut sum = 0;

  for game in &possible_games {
    sum += game;
  }

  let mut ordered = Vec::from_iter(possible_games);
  ordered.sort();

  println!("Part One: {sum}");
}

fn part_two(content: &String) {
  let re = Regex::new(r"\d+").unwrap();

  let mut result = 0;

  for line in content.lines() {
    let mut max_set = Set::new();

    let index = line.find(":").unwrap();

    let line = line[index + 1..].to_string();

    for set_str in line.split(";").map(|set| set.trim()) {
      for cube in set_str.split(",") {
        let quantity = re
          .captures_iter(cube)
          .next()
          .unwrap()
          .get(0)
          .unwrap()
          .as_str()
          .parse::<i32>()
          .unwrap();

        if cube.contains("red") && max_set.red < quantity {
          max_set.red = quantity;
        } else if cube.contains("green") && max_set.green < quantity {
          max_set.green = quantity;
        } else if cube.contains("blue") && max_set.blue < quantity {
          max_set.blue = quantity;
        }
      }
    }

    result += max_set.red * max_set.green * max_set.blue;
  }

  println!("Part Two: {result}");
}
