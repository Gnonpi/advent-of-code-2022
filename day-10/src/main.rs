use reqwest::blocking::Client;
use reqwest::header;
use std::time::Duration;

const DAY: u8 = 10;

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
enum Command {
    AddX(isize),
    Noop,
}

impl From<String> for Command {
    fn from(item: String) -> Self {
        if item.starts_with("addx") {
            let (_left, right) = item.split_once(' ').unwrap();
            let right_num = right.parse::<isize>().unwrap();
            Command::AddX(right_num)
        } else {
            Command::Noop
        }
    }
}

type AdventParsed = Vec<Command>;
type AdventResponse = isize;

fn parse_input(puzzle_input: String) -> AdventParsed {
    let mut parsed = vec![];
    for line in puzzle_input.lines() {
        if line.is_empty() {
            continue;
        }
        parsed.push(Command::from(line.trim().to_string()));
    }
    parsed
}

fn apply_commands(parsed: AdventParsed) -> Vec<isize> {
    // check what we're going to add
    let mut applied = vec![];
    for cmd in parsed {
        match cmd {
            Command::AddX(value) => {
                applied.push(0);
                applied.push(value);
            }
            Command::Noop => {
                applied.push(0);
            }
        }
    }
    // apply the modifications to a register
    let mut results = vec![1];
    for applied in applied {
        let previous = results.last().unwrap();
        results.push(previous + applied)
    }
    results
}

fn take_20_then_every_40(registers: Vec<isize>) -> Vec<(usize, isize)> {
    if registers.len() < 20 {
        panic!("too short");
    }
    let mut results = vec![];
    let offset = 20;
    let mut cnt = 0;
    let mut position = cnt * 40 + offset;
    while position < registers.len() {
        results.push((position, registers[position - 1]));
        cnt += 1;
        position = cnt * 40 + offset;
    }
    results
}

fn solve_one(parsed: AdventParsed) -> AdventResponse {
    let registers = apply_commands(parsed);
    let interesting_registers = take_20_then_every_40(registers);
    let multiplied_registers: Vec<isize> = interesting_registers
        .iter()
        .map(|x| (x.0 as isize) * x.1)
        .collect();
    let signal_strength = multiplied_registers.iter().sum();
    signal_strength
}

fn draw_crt(parsed: AdventParsed) -> String {
    let registers = apply_commands(parsed);
    let mut long_line: Vec<char> = vec![];
    // iterate over the registers
    // and compute each char
    for (i, reg) in registers.iter().enumerate() {
        let reg_mod = *reg;
        let i_isize = (i as isize) % 40;
        if (reg_mod == i_isize - 1) || (reg_mod == i_isize) || (reg_mod == i_isize + 1) {
            long_line.push('#')
        } else {
            long_line.push('.')
        }
    }
    // split the line into the 6 ones
    let split_lines = long_line.chunks_exact(40);
    let mut result = String::new();
    for chunk in split_lines {
        result.push_str(&(chunk.iter().collect::<String>() + "\n"));
    }

    result
}

fn solve_two(parsed: AdventParsed) -> String {
    draw_crt(parsed)
}

fn main() {
    read_cookie_value();
    let raw_input = get_puzzle_input();
    let parsed = parse_input(raw_input);
    let first_solution = solve_one(parsed.clone());
    println!("First solution: {:?}", first_solution);
    let second_solution = solve_two(parsed);
    println!("Second solution:\n{}", second_solution);
    // BJFRHRFU
}

#[cfg(test)]
mod day_test {
    use super::*;

    #[test]
    fn it_can_parse_command() {
        let addx = "addx -12".to_string();
        let result = Command::from(addx);
        assert_eq!(result, Command::AddX(-12));

        let noop = "noop".to_string();
        let result = Command::from(noop);
        assert_eq!(result, Command::Noop);
    }

    #[test]
    fn it_can_apply_commands() {
        let small_example = "noop
        addx 3
        addx -5"
            .to_string();
        let expected = vec![1, 1, 1, 4, 4, -1];
        let parsed = parse_input(small_example.to_string());
        let result = apply_commands(parsed);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_can_find_interesting_registers() {
        let parsed = parse_input(EXAMPLE.to_string());
        let registers = apply_commands(parsed);
        let interesting_registers = take_20_then_every_40(registers);
        let expected = vec![
            (20, 21),
            (60, 19),
            (100, 18),
            (140, 21),
            (180, 16),
            (220, 18),
        ];
        assert_eq!(interesting_registers, expected);
    }

    #[test]
    fn it_can_solve_example_part_1() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_one(parsed);
        assert_eq!(result, 13140);
    }

    #[test]
    fn it_can_draw_crt() {
        let parsed = parse_input(EXAMPLE.to_string());
        let drawn = draw_crt(parsed);
        println!("{}", drawn);
        assert_eq!(drawn, SCREEN_SECOND);
    }

    #[test]
    fn it_can_solve_example_part_2() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_two(parsed);
        // manually solved with the eye
        todo!();
    }

    const SCREEN_SECOND: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    const EXAMPLE: &str = "addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop";
}
