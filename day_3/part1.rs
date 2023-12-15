use std::path::Path;
use std::fs;

struct Number {
    value: usize,
    x: usize,
    y: usize,
}

fn main() {
    let file_path = Path::new("input.txt");
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");

    println!("Input:\n{contents}");

    let mut nums_vec: Vec<Number> = vec![];

    for (y, line) in contents.lines().enumerate() {
        let mut numbers_pre_concat = line
              .match_indices(|c: char| c.is_digit(10))
              .peekable();

        while numbers_pre_concat.peek() != None {
            let mut value = numbers_pre_concat.peek().unwrap().1.parse::<usize>().unwrap();
            let x = numbers_pre_concat.peek().unwrap().0;

            // This is garbage. Too bad!
            let mut index = numbers_pre_concat.next().unwrap().0;
            let mut next_index = numbers_pre_concat.peek().unwrap_or(&(usize::MAX, "")).0;
            while next_index == index + 1 && numbers_pre_concat.peek() != None {
                value = value * 10 + numbers_pre_concat.next().unwrap().1.parse::<usize>().unwrap();
                index = next_index;
                next_index = numbers_pre_concat.peek().unwrap_or(&(usize::MAX, "")).0;
            }

            let element = Number {
                value,
                x,
                y,
            };
            nums_vec.push(element);
        }
    }

    let mut sum = 0;
    
    let mut schematics: Vec<Vec<char>> = vec![];
    for line in contents.lines() {
        schematics.push(line.chars().collect());
    }

    'outer: for num in nums_vec {
        let length: usize = (num.value.ilog10() + 1) as usize;

        let valid_north = num.y > 0;
        let valid_east = num.x + length  < schematics[0].len();
        let valid_south = num.y + 1 < schematics.len();
        let valid_west = num.x > 0;

        for i in 0..length {
            // northern sections
            if valid_north {
                let character = schematics[num.y - 1][num.x + i];

                if !character.is_digit(10) && character != '.' {
                    sum += num.value;
                    continue 'outer;
                }
            }

            // southern sections
            if valid_south {
                let character = schematics[num.y + 1][num.x + i];

                if !character.is_digit(10) && character != '.' {
                    sum += num.value;
                    continue 'outer;
                }
            }
        }

        if valid_east
            && !schematics[num.y][num.x + length].is_digit(10)
            && schematics[num.y][num.x + length] != '.' {
                sum += num.value;
                continue 'outer;
        }

        
        if valid_west
            && !schematics[num.y][num.x - 1].is_digit(10)
            && schematics[num.y][num.x - 1] != '.' {
                sum += num.value;
                continue 'outer;
        }

        if valid_north && valid_east
            && !schematics[num.y - 1][num.x + length].is_digit(10)
            && schematics[num.y - 1][num.x + length] != '.' {
                sum += num.value;
                continue 'outer;
        }

        if valid_north && valid_west
            && !schematics[num.y - 1][num.x - 1].is_digit(10)
            && schematics[num.y - 1][num.x - 1] != '.' {
                sum += num.value;
                continue 'outer;
        }

        if valid_south && valid_east
            && !schematics[num.y + 1][num.x + length].is_digit(10)
            && schematics[num.y + 1][num.x + length] != '.' {
                sum += num.value;
                continue 'outer;
        }

        if valid_south && valid_west
            && !schematics[num.y + 1][num.x - 1].is_digit(10)
            && schematics[num.y + 1][num.x - 1] != '.' {
                sum += num.value;
                continue 'outer;
        }
    }

    println!("Output:\n{sum}");
}
