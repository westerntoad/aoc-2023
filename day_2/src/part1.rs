use std::fs;

struct Round {
    reds: u32,
    greens: u32,
    blues: u32,
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
    valid: bool,
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read file.");

    println!("Input: \n{contents}");

    let mut games: Vec<Game> = vec![];

    for (i, line) in contents.lines().enumerate() {
        let id = i as u32 + 1;
        let mut rounds: Vec<Round> = vec![];

        let round_strings: Vec<&str> = line[8..].split(';').collect();

        for round in &round_strings {
            rounds.push(round_from_string(round));
        }

        let valid = true;

        let game = Game { id, rounds, valid };

        games.push(game);
    }

    let mut sum = 0;

    for mut game in games {
        for round in game.rounds {
            if round.reds > 12 || round.greens > 13 || round.blues > 14 {
                game.valid = false;
                break;
            }
        }

        if game.valid {
            sum += game.id;
        }
    }

    println!("\n\nOutput: {sum}");
}

fn round_from_string(line: &str) -> Round {
    let mut reds: u32 = 0;
    let mut greens: u32 = 0;
    let mut blues: u32 = 0;

    let mut iter = line
        .chars()
        .filter(|x| *x != ' ' && *x != ',' && *x != ':')
        .peekable();

    while iter.peek().is_some() {
        let mut value_as_string = String::new();
        while iter.peek().unwrap().is_numeric() {
            value_as_string.push(iter.next().unwrap());
        }
        let value: u32 = value_as_string.parse::<u32>().unwrap_or(0);

        match iter.next().unwrap() {
            'r' => {
                reds = value;
                iter.nth(1);
            }
            'g' => {
                greens = value;
                iter.nth(3);
            }
            'b' => {
                blues = value;
                iter.nth(2);
            }
            _ => panic!(),
        }
    }

    Round {
        reds,
        greens,
        blues,
    }
}
