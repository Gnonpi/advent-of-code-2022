use reqwest::blocking::Client;
use reqwest::header;
use std::collections::HashSet;
use std::time::Duration;
mod point;
use point::{coverage_within_manhattan, coverage_within_manhattan_at_y, manhattan_distance, Point};

const DAY: u8 = 15;

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

type AdventParsed = Vec<(Point, Point, usize)>;
type AdventResponse = usize;

fn parse_line(line: String) -> (Point, Point, usize) {
    let (sensor_line, beacon_line) = line.split_once(":").unwrap();
    let sensor = Point::from(sensor_line.to_string());
    let beacon = Point::from(beacon_line.to_string());
    let distance = manhattan_distance(&sensor, &beacon);
    (sensor, beacon, distance)
}

fn parse_input(puzzle_input: String) -> AdventParsed {
    let mut result = vec![];
    for line in puzzle_input.lines() {
        if line.is_empty() {
            continue;
        }
        let parsed_line = parse_line(line.to_string());
        result.push(parsed_line);
    }
    result
}

fn count_positions_beacon_cannot_exist_at_y(signals: AdventParsed, level_y: isize) -> AdventResponse {
    // filter out sensors that are too high or too low
    let filtered_sensors: Vec<(Point, Point, usize)> = signals
        .iter()
        .filter(|(s, _, d)| (s.y - level_y) <= (*d as isize))
        .cloned()
        .collect();
    let mut covered_points = HashSet::new();
    // for each sensor
    for (sensor, _, dist) in filtered_sensors {
        // find all covered points
        let sensor_coverage = coverage_within_manhattan_at_y(&sensor, dist, level_y);
        covered_points = covered_points.union(&sensor_coverage).cloned().collect();
    }
    // remove beacons on that line
    for (_, beacon, _) in signals {
        if beacon.y == level_y {
            covered_points.remove(&beacon);
        }
    }
    covered_points.len()
}

fn solve_one(parsed: AdventParsed) -> AdventResponse {
    let level_y = 2_000_000;
    count_positions_beacon_cannot_exist_at_y(parsed, level_y)
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
    println!("Got input, start solving");
    let first_solution = solve_one(parsed.clone());
    println!("First solution: {:?}", first_solution);
    let second_solution = solve_two(parsed);
    println!("Second solution: {:?}", second_solution);
}

#[cfg(test)]
mod day_test {
    use super::*;

    const EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn it_can_parse_example() {
        let parsed = parse_input(EXAMPLE.to_string());
        let expected = vec![
            (Point { x: 2, y: 18 }, Point { x: -2, y: 15 }, 7),
            (Point { x: 9, y: 16 }, Point { x: 10, y: 16 }, 1),
        ];
        assert_eq!(parsed.len(), 14);
        assert_eq!(parsed[0], expected[0]);
        // assert_eq!(parsed, expected);
    }

    #[test]
    fn it_can_solve_example_part_1() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = count_positions_beacon_cannot_exist_at_y(parsed, 10);
        assert_eq!(result, 26);
    }

    #[test]
    fn it_can_solve_example_part_2() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_two(parsed);
        assert_eq!(result, 4);
    }
}
