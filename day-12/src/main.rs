use reqwest::blocking::Client;
use reqwest::header;
use std::time::Duration;

const DAY: u8 = 12;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

fn char_to_num(c: &char) -> usize {
    match c {
        &'S' => 0,
        &'E' => 27,
        _ => {
            (*c as usize) - 96
        }
    }
}

fn is_char_start(c: &char) -> bool {
    c == &'S'
}

fn is_char_end(c: &char) -> bool {
    c == &'E'
}

type AdventParsed = (Point, Point, Vec<Vec<usize>>);
type AdventResponse = u8;

fn parse_input(puzzle_input: String) -> AdventParsed {
    let mut result = vec![];
    let mut start = Point { x: 0, y: 0 };
    let mut end = Point { x: 0, y: 0 };
    for (i, line) in puzzle_input.lines().enumerate() {
        let mut row = vec![];
        let clean_line = line.trim();
        if clean_line.is_empty() {
            continue
        }
        for (j, letter) in clean_line.chars().enumerate() {
            if is_char_start(&letter) {
                start = Point { x: j, y: i };
            }
            if is_char_end(&letter) {
                end = Point { x: j, y: i };
            }
            row.push(char_to_num(&letter));
        }
        result.push(row);
    }
    (start, end, result)
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
    let parsed = parse_input(raw_input);
    let first_solution = solve_one(parsed.clone());
    println!("First solution: {:?}", first_solution);
    let second_solution = solve_two(parsed);
    println!("Second solution: {:?}", second_solution);
}

#[cfg(test)]
mod day_test {
    use super::*;

    const EXAMPLE: &str = "Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi";

    #[test]
    fn it_can_parse_example() {
        let parsed = parse_input(EXAMPLE.to_string());
        let expected = (
            Point {x: 0, y: 0},
            Point {x: 5, y: 2},
            vec![
                vec![0, 1, 2, 17, 16, 15, 14, 13],
                vec![1, 2, 3, 18, 25, 24, 24, 12],
                vec![1, 3, 3, 19, 26, 27, 24, 11],
                vec![1, 3, 3, 20, 21, 22, 23, 10],
                vec![1, 2, 4, 5, 6, 7, 8, 9],
            ]
        );
        assert_eq!(parsed.0, expected.0);
        assert_eq!(parsed.1, expected.1);
        assert_eq!(parsed.2, expected.2);
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

