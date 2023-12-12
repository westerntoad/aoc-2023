use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "sample.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");

    println!("Input:\n{contents}");

    let mut schematic: Vec<Vec<char>> = vec![];
    let mut nums_map: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    for (i, line) in contents.lines().enumerate() {
        schematic.push(line.chars().collect());

        let iter: Vec<&str> = line.split('.').filter(|&x| !x.is_empty()).collect();

        for word in iter {
            if word.parse::<usize>().is_err() {
                continue;
            }

            let num = word.parse::<usize>().unwrap();
            let mut coords: Vec<(usize, usize)> = vec![];

            let index = line.find(word).unwrap();

            for j in index..word.len() + index {
                let valid_north = i > 0;
                let valid_east = j < line.len() && j == word.len() + index - 1;
                let valid_south = i < contents.lines().count() - 1;
                let valid_west = j > 0 && j == index;

                // north
                if valid_north {
                    coords.push((j, i - 1));
                }

                // north-east
                if valid_north && valid_east {
                    coords.push((j + 1, i - 1));
                }

                // east
                if valid_east {
                    coords.push((j + 1, i));
                }

                // south-east
                if valid_south && valid_east {
                    coords.push((j + 1, i + 1));
                }

                // south
                if valid_south {
                    coords.push((j, i + 1));
                }

                // south-west
                if valid_south && valid_west {
                    coords.push((j - 1, i + 1));
                }

                // west
                if valid_west {
                    coords.push((j - 1, i));
                }

                // north-west
                if valid_north && valid_west {
                    coords.push((i - 1, j - 1));
                }
            }

            nums_map.insert(num, coords);
        }
    }

    let mut sum = 0;

    for num in nums_map.keys() {
        eprintln!("_____ num = {num}");
        for coord in nums_map.get(num).unwrap() {
            let schematic_coord = schematic[coord.1][coord.0];
            eprintln!(
                "coord, {:?}, {} digit = {}",
                coord,
                schematic_coord,
                schematic_coord.is_digit(10)
            );
            if !schematic_coord.is_digit(10) && schematic_coord != '.' {
                eprintln!("yep on {}", num);
                sum += num;
                break;
            }
        }
    }

    println!("Output:\n{sum}");
}
