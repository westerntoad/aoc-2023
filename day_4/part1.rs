use std::fs;

fn main() {
    let file_path = "input1.txt";
    let input = fs::read_to_string(file_path).expect("Failed to read file.");

    println!("Input: \n{input}");

    const BASE: i32 = 2;
    let mut output = 0;

    for line in input.lines() {
        let starting_index = line.find(':').unwrap() + 2;
        let mut given_numbers: Vec<i32> = vec![];

        // separate both sides of the card by '|' and further separate by spaces.
        // e.g:
        //  original: "41 48 83 86 17 | 83 86  6 31 17  9 48 53"
        //  Step 1  : "41 48 83 86 17", "83 86  6 31 17  9 48 53"
        //  Step 2  : [41, 48, 83, 86, 17], [86, 6, 31, 17, 9, 48, 53]

        // step 1 :
        let step_1: Vec<&str> = line[starting_index..].split('|').collect();
        // step 2 :
        let winning_numbers: Vec<i32> =
                step_1[0]
                .split(' ')
                .map(|x| -> i32 {
                    x.parse().unwrap_or(i32::MIN)
                })
                .filter(|x| x.is_positive())
                .collect();
        
        let given_numbers: Vec<i32> =
                step_1[1]
                .split(' ')
                .map(|x| -> i32 {
                    x.parse().unwrap_or(i32::MIN)
                })
                .filter(|x| x.is_positive())
                .collect();


        let mut points = 0;
        for num in winning_numbers {
            // if the number is found in given_numbers
            if given_numbers.iter().find(|&&x| x == num) != None {
                // improve score
                points += 1;
            }
        }
        if points != 0 {
            output += BASE.pow(points - 1);
        }
    }

    println!("{output}");
}
