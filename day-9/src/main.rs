use reqwest::blocking::Client;
use reqwest::header;
use std::collections::HashSet;
use std::time::Duration;

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

#[derive(Debug, PartialEq, Clone)]
struct Movement {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl From<String> for Movement {
    fn from(item: String) -> Self {
        let (left, right) = item.split_once(' ').unwrap();
        let mut x = 0;
        let mut y = 0;
        let right_num = right.parse::<isize>().unwrap();
        match left {
            "L" => {
                x = -right_num;
            }
            "R" => {
                x = right_num;
            }
            "U" => {
                y = right_num;
            }
            "D" => {
                y = -right_num;
            }
            _ => panic!("unknown letter: {:?}", left),
        }
        Movement { x, y }
    }
}

impl Movement {
    fn new(x: isize, y: isize) -> Self {
        Movement { x, y }
    }

    fn make_unary(&self) -> Movement {
        let xnum: usize = self.x.abs().try_into().unwrap();
        let ynum: usize = self.y.abs().try_into().unwrap();
        let mut new_x = 0;
        if xnum > 0 {
            new_x = self.x / (xnum as isize);
        }
        let mut new_y = 0;
        if ynum > 0 {
            new_y = self.y / (ynum as isize);
        }
        Movement {
            x: new_x,
            y: new_y,
        }
    }

    fn split_unit(&self) -> Vec<Movement> {
        let unary = self.make_unary();
        if self.x != 0 {
            let num: usize = self.x.abs().try_into().unwrap();
            vec![unary; num]
        } else {
            let num: usize = self.y.abs().try_into().unwrap();
            vec![unary; num]
        }
    }
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

fn apply_movement(point: &Point, mov: &Movement) -> Point {
    Point {
        x: point.x + mov.x,
        y: point.y + mov.y,
    }
}

fn is_head_adjacent(head: &Point, tail: &Point) -> bool {
    let sub = Movement {
        x: head.x - tail.x,
        y: head.y - tail.y,
    };
    (sub.x.abs() <= 1) && (sub.y.abs() <= 1)
}

fn follow_head(head: &Point, tail: &Point) -> Movement {
    if is_head_adjacent(head, tail) {
        return Movement {
            x: 0,
            y: 0,
        }
    }
    let sub = Movement {
        x: head.x - tail.x,
        y: head.y - tail.y,
    };
    sub.make_unary()
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

    const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

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
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_two(parsed);
        assert_eq!(result, 4);
    }
}
