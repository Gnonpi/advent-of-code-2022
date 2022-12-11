use reqwest::blocking::Client;
use reqwest::header;
use std::time::Duration;
mod monkey;
use monkey::{MonkeyArena, Monkey};

const DAY: u8 = 11;

fn read_cookie_value() {
    dotenvy::from_filename("../.env").unwrap();
}

fn input_url(day: u8) -> String {
    format!("https://adventofcode.com/2022/day/{}/input", day)
}

fn build_http_client() -> Client {
    let cookie_value = dotenvy::var("ADVENT_COOKIE").unwrap();
    // println!("{:#?}", cookie_value);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Cookie",
        header::HeaderValue::from_str(&cookie_value).unwrap(),
    );
    reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .default_headers(headers)
        .build()
        .unwrap()
}

fn get_puzzle_input() -> String {
    let client = build_http_client();
    let today_url = input_url(DAY);
    let resp = client.get(&today_url).send().unwrap().text().unwrap();

    // println!("{:#?}", resp);
    resp
}


type AdventParsed = MonkeyArena;
type AdventResponse = usize;

fn parse_input(puzzle_input: String) -> AdventParsed {
    let mut arena = MonkeyArena::new();
    for monkey_lines in puzzle_input.split("\n\n") {
        // println!("{:?}", monkey_lines);
        let monkey = Monkey::from(monkey_lines.to_string());
        arena.monkeys.push(monkey);
    }
    arena
}

fn solve_one(parsed: AdventParsed) -> AdventResponse {
    todo!();
}

fn solve_two(parsed: AdventParsed) -> AdventResponse {
    todo!();
}

fn main() {
    read_cookie_value();
    let raw_input = get_puzzle_input();
    let parsed = parse_input(raw_input.clone());
    let first_solution = solve_one(parsed);
    println!("First solution: {:?}", first_solution);
    let parsed = parse_input(raw_input);
    let second_solution = solve_two(parsed);
    println!("Second solution: {:?}", second_solution);
}

#[cfg(test)]
mod day_11_test {
    use super::*;

    const INPUT_MONKEYS: &str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3

  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0

  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3

  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1";

    #[test]
    fn it_can_parse_example() {
        let parsed = parse_input(INPUT_MONKEYS.to_string());
        // let expected = vec![
        // ];
        // assert_eq!(parsed.monkeys[0], expected[0]);
        assert_eq!(parsed.monkeys.len(), 4);
    }

    #[test]
    fn it_can_solve_example_part_1() {
        let parsed = parse_input(INPUT_MONKEYS.to_string());
        let result = solve_one(parsed);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_can_solve_example_part_2() {
        let parsed = parse_input(INPUT_MONKEYS.to_string());
        let result = solve_two(parsed);
        assert_eq!(result, 4);
    }
}
