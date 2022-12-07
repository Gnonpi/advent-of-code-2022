use reqwest::blocking::Client;
use reqwest::header;
use std::time::Duration;
mod command;
use command::{parse_commands, Command};

const DAY: u8 = 7;

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

type AdventParsed = Vec<Command>;
type AdventResponse = usize;

fn parse_input(puzzle_input: String) -> AdventParsed {
    parse_commands(puzzle_input)
}

fn solve_one(parsed: AdventParsed) -> AdventResponse {
    let mut result = 0;
    for x in parsed.iter() {
        todo!();
    }
    result
}

fn solve_two(parsed: AdventParsed) -> AdventResponse {
    let mut result = 0;
    for x in parsed.iter() {
        todo!();
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
mod day_test {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn it_can_parse_example() {
        let parsed = parse_input(EXAMPLE.to_string());
        let expected = vec![];
        assert_eq!(parsed[0], expected[0]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_can_solve_example_part_1() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_one(parsed);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_can_solve_example_part_2() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_two(parsed);
        assert_eq!(result, 4);
    }
}
