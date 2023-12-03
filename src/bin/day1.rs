use std::{fs, i32};

fn main() {
  let content = fs::read_to_string("./assets/day1.txt").unwrap();

  part_one(&content);
  part_two(&content);
}

fn part_one(content: &String) {
  let mut result = 0;

  for line in content.lines() {
    let mut left_number: Option<i32> = None;
    let mut right_number: Option<i32> = None;

    for char in line.chars() {
      if char.is_numeric() {
        if left_number.is_none() {
          left_number = Some(char.to_string().parse::<i32>().unwrap());
        }

        right_number = Some(char.to_string().parse::<i32>().unwrap());
      }
    }

    result += format!("{}{}", left_number.unwrap(), right_number.unwrap())
      .parse::<i32>()
      .unwrap();
  }

  println!("Part One: {}", result);
}

fn part_two(content: &String) {
  let map = vec![
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
  ];
  let mut result = 0;

  for line in content.lines() {
    let mut left_number: Option<i32> = None;
    let mut right_number: Option<i32> = None;
    let mut left_buf = String::new();
    let mut right_buf = String::new();

    for char in line.chars() {
      if char.is_numeric() {
        if left_number.is_none() {
          left_number = Some(char.to_string().parse::<i32>().unwrap());
        }

        right_number = Some(char.to_string().parse::<i32>().unwrap());
      } else if char.is_ascii_lowercase() {
        if left_number.is_none() {
          left_buf += &char.to_string();

          if let Some(number) = map.iter().find(|v| left_buf.contains(v.0)) {
            left_number = Some(number.1);
          }
        }

        right_buf += &char.to_string();

        if let Some(number) = map.iter().find(|v| right_buf.contains(v.0)) {
          right_number = Some(number.1);
          let index = right_buf.find(number.0);
          right_buf = right_buf[index.unwrap() + 1..].to_string();
        }
      }
    }

    let line_result = format!("{}{}", left_number.unwrap(), right_number.unwrap());

    result += line_result.parse::<i32>().unwrap();
  }

  println!("Part Two: {}", result);
}
