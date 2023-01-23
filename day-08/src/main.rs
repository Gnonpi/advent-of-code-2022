use reqwest::blocking::Client;
use reqwest::header;
use std::time::Duration;
mod forest;
use forest::Forest;

const DAY: u8 = 8;

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


type AdventParsed = Vec<Vec<u32>>;
type AdventResponse = u32;

fn parse_input(puzzle_input: String) -> AdventParsed {
    let mut result = vec![];
    for row in puzzle_input.split('\n').collect::<Vec<&str>>() {
        let clean_row = row.trim();
        if clean_row.is_empty() {
            continue
        }
        let chars = clean_row.chars().collect::<Vec<char>>();
        // to_digit returns a u32
        let line = chars.iter().map(|x| x.to_digit(10).unwrap() ).collect();
        result.push(line);
    }
    result
}

fn solve_one(parsed: AdventParsed) -> AdventResponse {
    let mut result = 0;
    let mut ft = Forest::from(parsed);
    ft.update_visibility();
    println!("Size: {:?}", ft.get_size());
    let mut cnt = 0;
    for t in ft.get_trees_flatten().iter() {
        cnt += 1;
        if t.is_any_visible() {
            result += 1;
        }
    }
    println!("cnt: {:?}", cnt);
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

    const EXAMPLE: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn it_can_parse_example() {
        let parsed = parse_input(EXAMPLE.to_string());
        let expected = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(parsed[0], expected[0]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_can_solve_example_part_1() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_one(parsed);
        assert_eq!(result, 21);
    }

    #[test]
    fn it_can_solve_example_part_2() {
        todo!();
        // let parsed = parse_input(EXAMPLE.to_string());
        // let result = solve_two(parsed);
        // assert_eq!(result, 4);
    }
}
