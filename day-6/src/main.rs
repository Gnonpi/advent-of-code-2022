use reqwest::blocking::Client;
use reqwest::header;
use std::time::Duration;
use std::collections::HashSet;

const DAY: u8 = 6;

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


type AdventParsed = String;
type AdventResponse = u32;

fn parse_input(puzzle_input: String) -> AdventParsed {
    puzzle_input
}

fn is_unique_char(sliced: &[char]) -> bool {
    let previous_len = sliced.len();
    let set_sliced: HashSet<&char> = HashSet::from_iter(sliced.iter());
    previous_len == set_sliced.len()
}

fn find_first_group_distinct(input_string: String, scan_size: usize) -> Option<u32> {
    let chars: Vec<char> = input_string.chars().collect();
    for i in 0..input_string.len() {
        let sliced = &chars[i..i + scan_size];
        if is_unique_char(sliced) {
            return Some((i + scan_size) as AdventResponse)
        }
    }
    None
}

fn solve_one(parsed: AdventParsed) -> AdventResponse {
    find_first_group_distinct(parsed, 4).unwrap()
}

fn solve_two(parsed: AdventParsed) -> AdventResponse {
    find_first_group_distinct(parsed, 14).unwrap()
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

    #[test]
    fn it_can_parse_example() {
        let parsed = parse_input("identity".to_string());
        let expected = String::from("identity");
        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_can_solve_example_part_1() {
        let cases = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for (input_str, expected) in cases.iter() {
            let result = solve_one(input_str.to_string());
            assert_eq!(result, *expected);
        }
    }

    #[test]
    fn it_can_solve_example_part_2() {
        let cases = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        for (input_str, expected) in cases.iter() {
            let result = solve_two(input_str.to_string());
            assert_eq!(result, *expected);
        }
    }
}
