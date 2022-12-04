pub fn solution1(data: String) -> i32 {
    let scores = get_scores(data);
    let result = scores.iter().sum();

    println!("=========================");
    println!("Solution1: {result}");
    println!("=========================");

    result
}

pub fn solution2(data: String) -> i32 {
    let scores = get_scores_v2(data);
    let result: i32 = scores.iter().sum();

    println!("=========================");
    println!("Solution2: {result}");
    println!("=========================");

    result
}

/////////////////////////////////////////////////

#[derive(Debug)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissor = 3
}

enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6
}

fn char_to_move(m: &str) -> Move {
    match m {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissor,
        _ => panic!("Invalid move {m}")
    }
}

fn get_round_outcome_value(m1: Move, m2: Move) -> i8 {
    match (m2 as i8) - (m1 as i8) {
        0 => Outcome::Draw as i8,
        1 | -2 => Outcome::Win as i8,
        _ => Outcome::Loss as i8
    }
}

fn get_scores(data: String) -> Vec<i32> {
    data.lines()
        .map(|line| {
            let chars: Vec<&str> = line.split_whitespace().collect();
            (char_to_move(chars[0]), char_to_move(chars[1]))
        })
        .map(|(m1, m2)| (m2 as i8) + get_round_outcome_value(m1, m2))
        .map(|score| score as i32)
        .collect()
}

fn char_to_outcome(m: &str) -> Outcome {
    match m {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Invalid outcome {m}")
    }
}

fn get_move_to_play(m1: Move, outcome: Outcome) -> Move {
    match outcome {
        Outcome::Loss => match m1 {
            Move::Rock => Move::Scissor,
            Move::Paper => Move::Rock,
            Move::Scissor => Move::Paper
        },
        Outcome::Draw => m1,
        Outcome::Win => match m1 {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissor,
            Move::Scissor => Move::Rock
        }
    }
}

fn get_scores_v2(data: String) -> Vec<i32> {
    data.lines()
        .map(|line| {
            let chars: Vec<&str> = line.split_whitespace().collect();
            (char_to_move(chars[0]), char_to_outcome(chars[1]))
        })
        .map(|(m1, outcome)| (outcome as i8) + (get_move_to_play(m1, outcome) as i8))
        .map(|score| score as i32)
        .collect()
}

/////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_file() -> String {
        let current_file = std::file!();
        let test_file = current_file.replace("mod.rs", "test.txt");
        return crate::read_file(&test_file).unwrap();
    }

    #[test]
    fn test_solution1() {
        let data = read_test_file();
        assert_eq!(15 + 7, solution1(data));
    }

    #[test]
    fn test_solution2() {
        let data = read_test_file();
        assert_eq!(12 + 2, solution2(data));
    }
}
