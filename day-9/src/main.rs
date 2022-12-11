use reqwest::blocking::Client;
use reqwest::header;
use std::collections::HashSet;
use std::time::Duration;
mod move_func;
mod movement;
mod point;
mod rope;
use move_func::{apply_movement, follow_head};
use movement::Movement;
use point::Point;
use rope::Rope;

const DAY: u8 = 9;

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

type AdventParsed = Vec<Movement>;
type AdventResponse = usize;

fn parse_input(puzzle_input: String) -> AdventParsed {
    let mut parsed = vec![];
    for line in puzzle_input.lines() {
        if line.is_empty() {
            continue;
        }
        parsed.push(Movement::from(line.trim().to_string()));
    }
    parsed
}

fn follow_movements(parsed: AdventParsed) -> HashSet<Point> {
    let mut result: HashSet<Point> = HashSet::new();
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    for movement in parsed {
        // println!(">> going {:?}", movement);
        let unaries = movement.split_unit();
        for unit_mov in unaries {
            // head is always moving
            head = apply_movement(&head, &unit_mov);
            // println!("Moving head to {:?}", head.clone());
            // first head movement
            let tail_mov = follow_head(&head, &tail);
            tail = apply_movement(&tail, &tail_mov);
            let _added = result.insert(tail.clone());
            // println!("tail {:?}", tail.clone());
            // if added {
            // println!("adding {:?}", tail.clone());
            // }
            // tail is moving if it's not the last move
            // if tail == head {
            //     panic!("tail over head: h:{:?} - t:{:?}", head.clone(), tail.clone());
            // }
        }
    }
    result
}

fn solve_one(parsed: AdventParsed) -> AdventResponse {
    let positions = follow_movements(parsed);
    positions.len()
}

fn solve_two(parsed: AdventParsed) -> AdventResponse {
    let mut rope = Rope::new(Point::new(0, 0), 10);
    for movement in parsed {
        rope.apply_movement(movement);
    }
    let tail_mem = rope.get_tail_memory();
    tail_mem.len()
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

    const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const LARGER_EXAMPLE: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn it_can_parse_example() {
        let parsed = parse_input(EXAMPLE.to_string());
        let expected = vec![
            Movement::new(4, 0),
            Movement::new(0, 4),
            Movement::new(-3, 0),
            Movement::new(0, -1),
            Movement::new(4, 0),
            Movement::new(0, -1),
            Movement::new(-5, 0),
            Movement::new(2, 0),
        ];
        assert_eq!(parsed[0], expected[0]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_can_follow_positions() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = follow_movements(parsed);
        let expected = HashSet::from([
            //
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
            Point::new(4, 1),
            Point::new(4, 2),
            //
            Point::new(4, 3),
            Point::new(0, 0),
            Point::new(3, 4),
            Point::new(2, 4),
            Point::new(3, 3),
            //
            Point::new(3, 2),
            Point::new(2, 2),
            Point::new(1, 2),
        ]);
        assert_eq!(result.len(), expected.len());
        // if result != expected {
        //     let sub = &expected - &result;
        //     println!("sub: {:?}", sub);
        //     let osub = &result - &expected;
        //     println!("osub: {:?}", osub);
        // }
        assert_eq!(result, expected);
    }

    #[test]
    fn it_can_solve_example_part_1() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_one(parsed);
        assert_eq!(result, 13);
    }

    #[test]
    fn it_can_solve_example_part_2() {
        let parsed = parse_input(LARGER_EXAMPLE.to_string());
        let result = solve_two(parsed);
        assert_eq!(result, 36);
    }
}
