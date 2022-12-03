use std::process;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");

    // first letter
    // A Rock
    // B Paper
    // C Scissors

    // second letter
    // X Rock => gives 1
    // Y Paper => gives 2
    // Z Scissors => gives 3

    // win bonus: 6
    // draw bonus: 3
    // loss bonus: 0


    let mut total: u32 = 0;
    for line in lines {
        if line.is_empty() { continue; }
        let mut points = 0;
        let set: Vec<&str> = line.split(" ").collect();
        let opponent = match set[0] {
            "A" => "ROCK",
            "B" => "PAPER",
            "C" => "SCISSORS",
            _ => process::exit(-1)
        };
        let own = match set[1] {
            "X" => "ROCK",
            "Y" => "PAPER",
            "Z" => "SCISSORS",
            _ => process::exit(-1)
        };
        let outcome = if opponent == "ROCK" && own == "PAPER" {
            "WON"
        } else if opponent == "PAPER" && own == "SCISSORS" {
            "WON"
        } else if opponent == "SCISSORS" && own == "ROCK" {
            "WON"
        } else if opponent == own {
            "DRAW"
        } else {
            "LOSS"
        };
        match own {
            "ROCK" => points += 1,
            "PAPER" => points += 2,
            "SCISSORS" => points += 3,
            _ => process::exit(-1),
        }
        if outcome == "WON" {
            points += 6
        } else if outcome == "DRAW" {
            points += 3
        }
        println!("own {}, opponent {}, outcome {},  points: {}", own, opponent, outcome, points);
        total += points;
    }
    return Some(total);
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n");

    // first letter
    // A Rock => gives 1
    // B Paper => gives 2
    // C Scissors => gives 3

    // second letter
    // X: need to lose
    // Y: need to draw
    // Z: need to win

    // win bonus: 6
    // draw bonus: 3
    // loss bonus: 0


    let mut total: u32 = 0;
    for line in lines {
        if line.is_empty() { continue; }
        let mut points = 0;
        let set: Vec<&str> = line.split(" ").collect();
        let opponent = match set[0] {
            "A" => "ROCK",
            "B" => "PAPER",
            "C" => "SCISSORS",
            _ => process::exit(-1)
        };
        let own = match set[1] {
            // lose
            "X" => match opponent {
                "ROCK" => "SCISSORS",
                "PAPER" => "ROCK",
                "SCISSORS" => "PAPER",
                _ => process::exit(-1)
            },
            // draw
            "Y" => opponent,
            // win
            "Z" => match opponent {
                "PAPER" => "SCISSORS",
                "SCISSORS" => "ROCK",
                "ROCK" => "PAPER",
                _ => process::exit(-1)
            },
            _ => process::exit(-1)
        };
        let outcome = if opponent == "ROCK" && own == "PAPER" {
            "WON"
        } else if opponent == "PAPER" && own == "SCISSORS" {
            "WON"
        } else if opponent == "SCISSORS" && own == "ROCK" {
            "WON"
        } else if opponent == own {
            "DRAW"
        } else {
            "LOSS"
        };
        match own {
            "ROCK" => points += 1,
            "PAPER" => points += 2,
            "SCISSORS" => points += 3,
            _ => process::exit(-1),
        }
        if outcome == "WON" {
            points += 6
        } else if outcome == "DRAW" {
            points += 3
        }
        println!("own {}, opponent {}, outcome {},  points: {}", own, opponent, outcome, points);
        total += points;
    }
    return Some(total);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
