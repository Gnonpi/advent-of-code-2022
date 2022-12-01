use reqwest::blocking::Client;
use dotenvy::dotenv;
use reqwest::header;
use std::time::Duration;
use std::env;

const DAY: u8 = 1;

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


fn parse_input(puzzle_input: String) -> Vec<Vec<u32>> {
    let mut result = vec![];
    let mut buffer = vec![];
    for row in puzzle_input.split("\n") {
        match row {
            "" => {
                result.push(buffer);
                buffer = vec![];
            },
            x => buffer.push(x.parse::<u32>().unwrap())
        }
    }
    result
}

fn solve_one(parsed: Vec<Vec<u32>>) -> u32 {
    // sum each elf
    let summed: Vec<u32> = parsed
        .into_iter()
        .map(|v| v.into_iter().sum())
        .collect();
    // find max value
    summed.iter().max().unwrap().clone()
}

fn solve_two(parsed: Vec<Vec<u32>>) -> u32 {
    // sum each elf
    let mut summed: Vec<u32> = parsed
        .into_iter()
        .map(|v| v.into_iter().sum())
        .collect();
    // sort
    summed.sort();
    // get top three
    summed.reverse();
    let top_three = &summed[0..3];
    // println!("{:?}", top_three);
    // sum top three
    top_three.iter().sum()
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
mod day1_test {
    use super::*;

    #[test]
    fn it_can_parse_example() {
        let example =
            String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n");
        let parsed = parse_input(example);
        let expected = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_can_solve_example_part_1() {
        let example =
            String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n");
        let parsed = parse_input(example);
        let result = solve_one(parsed);
        assert_eq!(result, 24000);
    }

    #[test]
    fn it_can_solve_example_part_2() {
        let example =
        String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n");
        let parsed = parse_input(example);
        let result = solve_two(parsed);
        assert_eq!(result, 45000);
    }
}
