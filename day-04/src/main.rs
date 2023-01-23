use reqwest::blocking::Client;
use reqwest::header;
use std::time::Duration;

const DAY: u8 = 4;

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
struct SectionInterval {
    lower: u32,
    upper: u32,
}
impl SectionInterval {
    fn size(&self) -> u32 {
        self.upper - self.lower
    }
}
// so many unwraps
impl From<String> for SectionInterval {
    fn from(item: String) -> Self {
        let (left, right) = item.split_once('-').unwrap();
        SectionInterval {
            lower: left.parse::<u32>().unwrap(),
            upper: right.parse::<u32>().unwrap(),
        }
    }
}

type AdventParsed = Vec<(SectionInterval, SectionInterval)>;
type AdventResponse = u32;

fn parse_input(puzzle_input: String) -> AdventParsed {
    let mut result = vec![];
    // go over each line
    for row in puzzle_input.split('\n') {
        if row.is_empty() {
            continue;
        }
        let (left, right) = row.split_once(',').unwrap();
        let left_section = SectionInterval::from(left.to_string());
        let right_section = SectionInterval::from(right.to_string());
        result.push((left_section, right_section));
    }
    result
}

fn is_small_included_in_big(sec_big: &SectionInterval, sec_small: &SectionInterval) -> bool {
    if sec_small.size() > sec_big.size() {
        return false;
    }
    if sec_small.lower < sec_big.lower {
        return false;
    }
    if sec_small.upper > sec_big.upper {
        return false;
    }
    true
}

fn is_one_included(sec_a: &SectionInterval, sec_b: &SectionInterval) -> bool {
    // we're also checking size in is_small_included_in_big
    // println!("{:?} - {:?}", sec_a, sec_b);
    // println!("size: {:?} - {:?}", sec_a.size(), sec_b.size());
    if sec_a.size() > sec_b.size() {
        is_small_included_in_big(sec_a, sec_b)
    } else {
        is_small_included_in_big(sec_b, sec_a)
    }
}

fn solve_one(parsed: AdventParsed) -> AdventResponse {
    let mut result = 0;
    for sections in parsed.iter() {
        if is_one_included(&sections.0, &sections.1) {
            result += 1;
        }
    }
    result
}

fn is_left_overlap_with_right(sec_left: &SectionInterval, sec_right: &SectionInterval) -> bool {
    (sec_left.upper >= sec_right.lower) || (sec_left.lower >= sec_right.upper)
}

fn pair_overlap(sec_a: &SectionInterval, sec_b: &SectionInterval) -> bool {
    if sec_a.lower <= sec_b.lower {
        is_left_overlap_with_right(sec_a, sec_b)
    } else {
        is_left_overlap_with_right(sec_b, sec_a)
    }
}

fn solve_two(parsed: AdventParsed) -> AdventResponse {
    let mut result = 0;
    for sections in parsed.iter() {
        if pair_overlap(&sections.0, &sections.1) {
            result += 1;
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
mod day4_test {
    use super::*;

    const EXAMPLE: &str = "2-4,6-8\n
2-3,4-5\n
5-7,7-9\n
2-8,3-7\n
6-6,4-6\n
2-6,4-8\n";

    #[test]
    fn it_can_parse_example() {
        let parsed = parse_input(EXAMPLE.to_string());
        let expected = vec![
            (
                SectionInterval { lower: 2, upper: 4 },
                SectionInterval { lower: 6, upper: 8 },
            ),
            (
                SectionInterval { lower: 2, upper: 3 },
                SectionInterval { lower: 4, upper: 5 },
            ),
            (
                SectionInterval { lower: 5, upper: 7 },
                SectionInterval { lower: 7, upper: 9 },
            ),
            (
                SectionInterval { lower: 2, upper: 8 },
                SectionInterval { lower: 3, upper: 7 },
            ),
            (
                SectionInterval { lower: 6, upper: 6 },
                SectionInterval { lower: 4, upper: 6 },
            ),
            (
                SectionInterval { lower: 2, upper: 6 },
                SectionInterval { lower: 4, upper: 8 },
            ),
        ];
        assert_eq!(parsed[0], expected[0]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_can_check_inclusion() {
        // included
        let res = is_small_included_in_big(
            &SectionInterval { lower: 1, upper: 4 },
            &SectionInterval { lower: 2, upper: 3 },
        );
        assert_eq!(res, true);
        // too big
        let res = is_small_included_in_big(
            &SectionInterval { lower: 1, upper: 5 },
            &SectionInterval { lower: 4, upper: 6 },
        );
        assert_eq!(res, false);
        // shifted
        let res = is_small_included_in_big(
            &SectionInterval { lower: 1, upper: 3 },
            &SectionInterval { lower: 4, upper: 6 },
        );
        assert_eq!(res, false);
    }

    #[test]
    fn it_find_inclusion() {
        let parsed = parse_input(EXAMPLE.to_string());
        assert_eq!(is_one_included(&parsed[0].0, &parsed[0].1), false);
        assert_eq!(is_one_included(&parsed[1].0, &parsed[1].1), false);
        assert_eq!(is_one_included(&parsed[2].0, &parsed[2].1), false);
        assert_eq!(is_one_included(&parsed[3].0, &parsed[3].1), true);
        assert_eq!(is_one_included(&parsed[4].0, &parsed[4].1), true);
        assert_eq!(is_one_included(&parsed[5].0, &parsed[5].1), false);
    }

    #[test]
    fn it_can_solve_example_part_1() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_one(parsed);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_can_find_overlap() {
        let parsed = parse_input(EXAMPLE.to_string());
        assert_eq!(pair_overlap(&parsed[0].0, &parsed[0].1), false);
        assert_eq!(pair_overlap(&parsed[1].0, &parsed[1].1), false);
        assert_eq!(pair_overlap(&parsed[2].0, &parsed[2].1), true);
        assert_eq!(pair_overlap(&parsed[3].0, &parsed[3].1), true);
        assert_eq!(pair_overlap(&parsed[4].0, &parsed[4].1), true);
        assert_eq!(pair_overlap(&parsed[5].0, &parsed[5].1), true);
    }

    #[test]
    fn it_can_solve_example_part_2() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_two(parsed);
        assert_eq!(result, 4);
    }
}
