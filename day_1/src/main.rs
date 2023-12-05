use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");

    println!("Input: \n{contents}");

    let mut sum: u32 = 0;
    let mut nums: Vec<Vec<u32>> = vec![];

    let dict = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for (i, line) in contents.lines().enumerate() {
        nums.push(vec![]);

        for (j, character) in line.chars().enumerate() {
            if character.is_numeric() {
                nums[i].push(character.to_digit(10).unwrap());
            } else {
                for key in dict.keys() {
                    let substring = &line.get(j..key.len() + j).unwrap_or("");
                    if substring == key {
                        nums[i].push(dict.get(key).unwrap().clone());
                        break;
                    }
                }
            }
        }
    }

    for mut line in nums {
        let first: u32 = line.get(0).copied().unwrap_or(0);
        let last: u32 = line.last_mut().copied().unwrap_or(0);

        println!("{}", first * 10 + last);

        sum += first * 10 + last;
    }

    println!("{sum}");
}
