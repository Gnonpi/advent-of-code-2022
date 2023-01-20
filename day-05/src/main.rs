use reqwest::blocking::Client;
use reqwest::header;
use std::time::Duration;
mod warehouse;
mod crane_instruction;
use warehouse::WarehouseState;
use crane_instruction::CraneInstruction;

const DAY: u8 = 5;

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

#[derive(Debug, PartialEq, Default, Clone)]
struct WareHouseAndInstructions<T> {
    initial: WarehouseState<T>,
    instructions: Vec<CraneInstruction>,
}

type AdventParsed = WareHouseAndInstructions<char>;
type AdventResponse = String;

fn parse_input(puzzle_input: String) -> AdventParsed {
    let mut buffer_warehouse = vec![];
    let mut buffer_instr = vec![];
    let mut end_warehouse = false;
    for line in puzzle_input.split('\n').collect::<Vec<&str>>().iter() {
        if line.is_empty() && end_warehouse == false {
            end_warehouse = true;
        }
        if end_warehouse {
            if !line.is_empty() {
                buffer_instr.push(line.clone());
            }
        } else {
            buffer_warehouse.push(line.clone());
        }
    }
    // println!("Parsing instructions");
    let warehouse = WarehouseState::from(
        String::from_iter(
            buffer_warehouse.into_iter().map(
                |x: &str| x.to_string() + "\n"
            )
        )
    );
    let instructions = buffer_instr.into_iter()
        .map(|s: &str| CraneInstruction::from(s.to_string()))
        .collect();
    WareHouseAndInstructions {
        initial: warehouse,
        instructions
    }
}

fn solve_one(parsed: AdventParsed) -> AdventResponse {
    let mut warehouse = parsed.initial.clone();
    for instr in parsed.instructions.iter() {
        warehouse.execute_command(instr.clone());
    }
    let tops = warehouse.get_tops();
    String::from_iter(tops.iter())
}

fn solve_two(parsed: AdventParsed) -> AdventResponse {
    let mut warehouse = parsed.initial.clone();
    for instr in parsed.instructions.iter() {
        warehouse.execute_command_9001(instr.clone());
    }
    let tops = warehouse.get_tops();
    String::from_iter(tops.iter())
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

    const EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn it_can_parse_example() {
        let parsed = parse_input(EXAMPLE.to_string());
        let s = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";
        let expected = WareHouseAndInstructions {
            initial: WarehouseState::from(s.to_string()),
            instructions: vec![
                CraneInstruction::from("move 1 from 2 to 1".to_string()),
                CraneInstruction::from("move 3 from 1 to 3".to_string()),
                CraneInstruction::from("move 2 from 2 to 1".to_string()),
                CraneInstruction::from("move 1 from 1 to 2".to_string())
            ],
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_can_solve_example_part_1() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_one(parsed);
        assert_eq!(result, "CMZ".to_string());
    }

    #[test]
    fn it_can_solve_example_part_2() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_two(parsed);
        assert_eq!(result, "MCD".to_string());
    }
}
