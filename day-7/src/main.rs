use reqwest::blocking::Client;
use reqwest::header;
use std::time::Duration;
mod command;
mod file_tree;
use command::{parse_commands, Command};
use file_tree::{FileTree, File};

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
    let ft = FileTree::from(parsed);
    let dirs = ft.get_directories();
    let mut result = 0;
    for dir in dirs {
        if dir.total_size <= 100000 {
            result += dir.total_size;
        }
    }
    result
}

fn solve_two(parsed: AdventParsed) -> AdventResponse {
    let ft = FileTree::from(parsed);
    let dirs = ft.get_directories();
    let total_space = 70000000;
    let needed_space = 30000000;
    // size of outermost directory
    let files = ft.get_directories();
    let root_folder = files.iter().filter(|x| x.name == "/").collect::<Vec<&File>>()[0];
    let outermost_size = root_folder.total_size;
    // println!("outermost: {:?}", outermost_size);
    // min size = needed - (total - outermost)
    let unused_space = total_space - outermost_size;
    let min_size = needed_space - unused_space;
    // println!("min_size: {:?}", min_size);
    // iterate over folders
    let mut result = outermost_size;
    for dir in files.iter() {
        // println!("dir: {:?}", dir.total_size);
        if dir.total_size >= min_size && dir.total_size < result  {
            result = dir.total_size;
        }
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

    const EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn it_can_solve_example_part_1() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_one(parsed);
        assert_eq!(result, 95437);
    }

    #[test]
    fn it_can_solve_example_part_2() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_two(parsed);
        assert_eq!(result, 24933642);
    }
}
