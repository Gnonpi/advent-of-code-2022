use reqwest::blocking::Client;
use reqwest::header;
use std::time::Duration;

mod point;
use point::Point;
mod grid;
use grid::{FallingGrid, GrainStatus};

const DAY: u8 = 14;

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


type AdventParsed = Vec<Vec<Point>>;
type AdventResponse = usize;

const STARTING_FALL: Point = Point { x: 500, y: 0 };


fn parse_line(line: String) -> Vec<Point> {
    line.split(" -> ").into_iter().map(|s| Point::from(s.to_string())).collect()
}


fn parse_input(puzzle_input: String) -> AdventParsed {
    let mut result = vec![];
    for line in puzzle_input.lines() {
        if line.is_empty() {
            continue;
        }
        result.push(parse_line(line.to_string()));
    }
    result
}

fn solve_one(parsed: AdventParsed) -> AdventResponse {
    // Create Grid with walls
    let mut fg = FallingGrid::default();
    for line in parsed.iter() {
        let mut previous = line[0];
        for point in line.iter().cloned() {
            if previous == point {
                continue
            }
            fg.add_line(previous, point);
            previous = point;
        }
    }
    // Make grains fall
    let mut last_status = GrainStatus::Stopped;
    let mut count_grain: usize = 0;
    while last_status == GrainStatus::Stopped {
        last_status = fg.fall_one_sand(STARTING_FALL.clone());
        if last_status == GrainStatus::Stopped {
            count_grain = count_grain + 1;
        }
    }
    count_grain
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

    const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn it_can_parse_single_line() {
        let line = String::from("498,4 -> 498,6 -> 496,6");
        let result = parse_line(line);
        let expected = vec![
            Point {x: 498, y: 4},
            Point {x: 498, y: 6},
            Point {x: 496, y: 6},
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_can_parse_example() {
        let parsed = parse_input(EXAMPLE.to_string());
        let expected = vec![
            vec![
                Point {x: 498, y: 4},
                Point {x: 498, y: 6},
                Point {x: 496, y: 6},
            ],
            vec![
                Point {x: 503, y: 4},
                Point {x: 502, y: 4},
                Point {x: 502, y: 9},
                Point {x: 494, y: 9},
            ],
        ];
        assert_eq!(parsed[0], expected[0]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_can_solve_example_part_1() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_one(parsed);
        assert_eq!(result, 24);
    }

    #[test]
    fn it_can_solve_example_part_2() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_two(parsed);
        assert_eq!(result, 4);
    }
}
