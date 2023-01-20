use reqwest::blocking::Client;
use dotenvy::dotenv;
use reqwest::header;
use std::time::Duration;
use std::env;

const DAY: u32 = 2;

fn read_cookie_value() {
    dotenvy::from_filename("../.env").unwrap();
}

fn input_url(day: u32) -> String {
    format!("https://adventofcode.com/2022/day/{}/input", day)
}


fn build_http_client() -> Client {
    let cookie_value = dotenvy::var("ADVENT_COOKIE").unwrap();
    // println!("{:#?}", cookie_value);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Cookie", 
        header::HeaderValue::from_str(&cookie_value).unwrap()
    );
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .default_headers(headers)
        .build().unwrap();
    client
}


fn get_puzzle_input() -> String {
    let client = build_http_client();
    let today_url = input_url(DAY);
    let resp = client.get(&today_url).send().unwrap().text().unwrap();

    // println!("{:#?}", resp);
    resp
}

type AdventParsed = Vec<(u32, u32)>;
type AdventResponse = u32;

fn letter_to_int(letter: char) -> u32 {
    match letter {
        'A' => 11,
        'B' => 12,
        'C' => 13,
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Unknown letter: {:?}", letter)
    }
}

fn parse_input(puzzle_input: String) -> AdventParsed {
    let mut result = vec![];
    for row in puzzle_input.split("\n") {
        // println!("{:?}", row);
        if row == "" {
            continue
        }
        let chars: Vec<char> = row.chars().collect();
        result.push(
            (letter_to_int(chars[0]), letter_to_int(chars[2]))
        );
    }
    result
}

#[derive(Debug)]
enum RoundOutcome {
    Lose,
    Win,
    Draw,
}

fn compute_round_outcome(round: &(u32, u32)) -> RoundOutcome {
    // Draw case
    if round.0 == (round.1 + 10) {
        return RoundOutcome::Draw
    }
    match round.0 {
        // A: Rock
        11 => {
            match round.1 {
                2 => RoundOutcome::Win,
                3 => RoundOutcome::Lose,
                _ => panic!(),
            }
        }, 
        // B: Paper
        12 => {
            match round.1 {
                1 => RoundOutcome::Lose,
                3 => RoundOutcome::Win,
                _ => panic!(),
            }
        },
        // C: Scissor
        13 => {
            match round.1 {
                1 => RoundOutcome::Win,
                2 => RoundOutcome::Lose,
                _ => panic!(),
            }
        },
        _ => panic!()
    }
}

fn compute_round_score(round: &(u32, u32)) -> u32 {
    // println!("{:?}", round);
    let outcome = compute_round_outcome(round);
    match outcome {
        RoundOutcome::Win => {
            // win: 6pts + value
            6 + round.1
        },
        RoundOutcome::Draw => {
            // draw: 3pts + value
            3 + round.1
        },
        RoundOutcome::Lose => {
            // lose: value
            round.1
        }
    }
}

fn solve_one(parsed: AdventParsed) -> AdventResponse {
    let mut result = 0;
    for round in parsed.iter() {
        // let ok = compute_round_score(&round.clone());
        // println!("Round {:?} - {:?}", round.clone(), ok);
        result += compute_round_score(round);
    }
    result
}


fn get_winner(sign: &u32) -> u32 {
    match sign {
        // Rock -> Paper
        1 => 2,
        // Paper -> Scissor
        2 => 3,
        // Scissor -> Rock
        3 => 1,
        _ => panic!()
    }
}


fn find_counter(opponent: &u32, outcome: RoundOutcome) -> u32 {
    // println!("opponent: {:?} - outcome: {:?}", opponent, outcome);
    match outcome {
        // draw: same as opponent
        RoundOutcome::Draw => compute_round_score(&(*opponent, opponent - 10)),
        RoundOutcome::Lose => compute_round_score(
            // twice win gives you loser
            &(*opponent, get_winner(&get_winner(&(opponent - 10))))
        ),
        RoundOutcome::Win => compute_round_score(
            &(*opponent, get_winner(&(opponent - 10)))
        ),
        _ => panic!()
    }
}

fn compute_counter(round: &(u32, u32)) -> u32 {
    match round.1 {
        // X: lose
        1 => find_counter(&round.0, RoundOutcome::Lose),
        // Y: draw
        2 => find_counter(&round.0, RoundOutcome::Draw),
        // Z: win
        3 => find_counter(&round.0, RoundOutcome::Win),
        _ => panic!()
    }
}

fn solve_two(parsed: AdventParsed) -> AdventResponse {
    let mut result = 0;
    for round in parsed.iter() {
        result += compute_counter(round);
    }
    result
}

fn main() {
    read_cookie_value();
    let raw_input = get_puzzle_input();
    let parsed = parse_input(raw_input);
    let first_solution = solve_one(parsed.clone());
    println!("First solution: {:?}", first_solution);
    let second_solution = solve_two(parsed);
    println!("Second solution: {:?}", second_solution);
}

#[cfg(test)]
mod day2_test {
    use super::*;

    #[test]
    fn it_can_parse_example() {
        let example = String::from("A Y\nB X\nC Z");
        let parsed = parse_input(example);
        let expected = vec![
            (11, 2),
            (12, 1),
            (13, 3),
        ];
        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_can_compute_score() {
        let a_y = compute_round_score(&(11, 2));
        assert_eq!(a_y, 8);
        let b_x = compute_round_score(&(12, 1));
        assert_eq!(b_x, 1);
        let c_z = compute_round_score(&(13, 3));
        assert_eq!(c_z, 6);
    }

    #[test]
    fn it_can_solve_example_part_1() {
        let example = String::from("A Y\nB X\nC Z");
        let parsed = parse_input(example);
        let result = solve_one(parsed);
        assert_eq!(result, 15);
    }

    #[test]
    fn it_can_find_counter() {
        let a_y = compute_counter(&(11, 2));
        assert_eq!(a_y, 4);
        let b_x = compute_counter(&(12, 1));
        assert_eq!(b_x, 1);
        let c_z = compute_counter(&(13, 3));
        assert_eq!(c_z, 7);
    }

    #[test]
    fn it_can_solve_example_part_2() {
        let example = String::from("A Y\nB X\nC Z");
        let parsed = parse_input(example);
        let result = solve_two(parsed);
        assert_eq!(result, 12);
    }
}
