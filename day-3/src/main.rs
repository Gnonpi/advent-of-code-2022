use reqwest::blocking::Client;
use reqwest::header;
use std::collections::HashSet;
use std::time::Duration;

const DAY: u8 = 3;

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
struct RuckSack {
    left: Vec<char>,
    right: Vec<char>,
}
impl RuckSack {
    fn fuse_compartments(&self) -> Vec<char> {
        [self.left.clone(), self.right.clone()].concat()
    }
}

type AdventParsed = Vec<RuckSack>;
type AdventResponse = u32;

fn parse_input(puzzle_input: String) -> AdventParsed {
    let mut result = vec![];
    // go over each line
    for row in puzzle_input.split('\n') {
        if row.is_empty() {
            continue;
        }
        // split into 2
        let middle_idx = row.len() / 2;
        let (l_split, r_split) = row.split_at(middle_idx);
        // add to RuckSack
        let r = RuckSack {
            left: l_split.chars().collect(),
            right: r_split.chars().collect(),
        };
        result.push(r)
    }
    result
}

fn get_letter_priority(letter: char) -> AdventResponse {
    match letter.is_lowercase() {
        // Lowercase: ascii - 96
        true => (letter as u32) - 96,
        // Uppercase: ascii - 64 + 26
        false => (letter as u32) - 64 + 26,
    }
}

fn find_duplicate_char(group_of_chars: Vec<&Vec<char>>) -> char {
    let mut set_intersect: HashSet<char> = HashSet::new();
    for group_vec in group_of_chars.iter() {
        let group_set: HashSet<char> = HashSet::from_iter(group_vec.iter().cloned());
        if set_intersect.is_empty() {
            set_intersect = group_set;
        } else {
            let intersect = set_intersect.intersection(&group_set);
            set_intersect = HashSet::from_iter(intersect.cloned());
        }
    }
    if set_intersect.len() != 1 {
        println!(
            "Intersection set doesn't contain a single element: {:?}",
            set_intersect
        );
    }
    let commons: Vec<char> = set_intersect.drain().collect();
    commons[0]
}

fn solve_one(parsed: AdventParsed) -> AdventResponse {
    let mut result = 0;
    for rucksack in parsed.iter() {
        let duplicated = find_duplicate_char(vec![&rucksack.left, &rucksack.right]);
        let dup_value = get_letter_priority(duplicated);
        result += dup_value;
    }
    result
}

fn solve_two(parsed: AdventParsed) -> AdventResponse {
    let mut result = 0;
    // iter over groups of three
    for three_rucksacks in parsed.rchunks(3) {
        let common_letter = find_duplicate_char(vec![
            &three_rucksacks[0].fuse_compartments(),
            &three_rucksacks[1].fuse_compartments(),
            &three_rucksacks[2].fuse_compartments(),
        ]);
        let common_value = get_letter_priority(common_letter);
        result += common_value;
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
mod day1_test {
    use super::*;

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\n
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n
PmmdzqPrVvPwwTWBwg\n
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n
ttgJtRGJQctTZtZT\n
CrZsJsPPZsGzwwsLwLmpwMDw\n";

    #[test]
    fn it_can_parse_example() {
        let parsed = parse_input(EXAMPLE.to_string());
        let expected = vec![
            RuckSack {
                left: "vJrwpWtwJgWr".chars().collect(),
                right: "hcsFMMfFFhFp".chars().collect(),
            },
            RuckSack {
                left: "jqHRNqRjqzjGDLGL".chars().collect(),
                right: "rsFMfFZSrLrFZsSL".chars().collect(),
            },
            RuckSack {
                left: "PmmdzqPrV".chars().collect(),
                right: "vPwwTWBwg".chars().collect(),
            },
            RuckSack {
                left: "wMqvLMZHhHMvwLH".chars().collect(),
                right: "jbvcjnnSBnvTQFn".chars().collect(),
            },
            RuckSack {
                left: "ttgJtRGJ".chars().collect(),
                right: "QctTZtZT".chars().collect(),
            },
            RuckSack {
                left: "CrZsJsPPZsGz".chars().collect(),
                right: "wwsLwLmpwMDw".chars().collect(),
            },
        ];
        assert_eq!(parsed[0].left, expected[0].left);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_can_get_priority() {
        assert_eq!(get_letter_priority('p'), 16);
        assert_eq!(get_letter_priority('L'), 38);
        assert_eq!(get_letter_priority('P'), 42);
        assert_eq!(get_letter_priority('v'), 22);
        assert_eq!(get_letter_priority('t'), 20);
        assert_eq!(get_letter_priority('s'), 19);
    }

    #[test]
    fn it_can_find_duplicate() {
        let first_row = RuckSack {
            left: "vJrwpWtwJgWr".chars().collect(),
            right: "hcsFMMfFFhFp".chars().collect(),
        };
        let result = find_duplicate_char(vec![&first_row.left, &first_row.right]);
        assert_eq!(result, 'p');
    }

    #[test]
    fn it_can_solve_example_part_1() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_one(parsed);
        assert_eq!(result, 157);
    }

    #[test]
    fn it_can_solve_example_part_2() {
        let parsed = parse_input(EXAMPLE.to_string());
        let result = solve_two(parsed);
        assert_eq!(result, 70);
    }
}
