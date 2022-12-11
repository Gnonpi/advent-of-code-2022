use regex::Regex;
use std::fmt::Debug;
use std::fmt::Formatter;
use core::fmt::Error;

type MonkeyNumber = usize;
type WorryItem = usize;

#[derive(Debug)]
pub(crate) struct MonkeyArena {
    round: usize,
    pub(crate) monkeys: Vec<Monkey>,
}

impl MonkeyArena {
    pub(crate) fn new() -> Self {
        MonkeyArena {
            round: 0,
            monkeys: Vec::new()
        }
    }

    pub(crate) fn get_monkey_business(&self) -> Vec<usize> {
        let mut results = vec![];
        for i_monkey in 0..self.monkeys.len() {
            results.push(self.monkeys[i_monkey].inspect_count)
        }
        results
    }

    pub(crate) fn play_round(&mut self) {
        for i_monkey in 0..self.monkeys.len() {
            let processed = self.monkeys[i_monkey].process_items();
            for res in processed {
                let (dest, item) = res;
                self.monkeys[dest].items.push(item);
            }
        }
        self.round += 1;
    }
}

pub(crate) struct Monkey {
    number: MonkeyNumber,
    items: Vec<WorryItem>,
    inspect_count: usize,
    operation: Box<dyn Fn(WorryItem) -> WorryItem>,
    condition: Box<dyn Fn(WorryItem) -> bool>,
    send_true: MonkeyNumber,
    send_false: MonkeyNumber,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Monkey")
            .field("number", &self.number)
            .field("items", &self.items)
            .field("inspect_count", &self.inspect_count)
            .field("send_true", &self.send_true)
            .field("send_false", &self.send_false)
            .finish()
    }
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number &&
        self.items == other.items &&
        self.inspect_count == other.inspect_count &&
        self.send_true == other.send_true &&
        self.send_false == other.send_false
    }
}


fn op_creator(lh: String, operator: String, rh: String) -> Box<dyn Fn(WorryItem) -> WorryItem> {
    if rh == "old" {
        match operator.as_str() {
            "+" => return Box::new(move |o| o + o),
            "*" => return Box::new(move |o| o * o),
            _ => panic!("no old operator"),
        }
    }
    let rh_num = rh.parse::<WorryItem>().unwrap();
    match operator.as_str() {
        "+" => return Box::new(move |o| o + rh_num.clone()),
        "*" => return Box::new(move |o| o * rh_num.clone()),
        _ => panic!("no old operator"),
    }
}

fn cond_creator(num: usize) -> Box<dyn Fn(WorryItem) -> bool> {
    Box::new(move |x| { (x % num) == 0 })
}

impl From<String> for Monkey {
    fn from(item: String) -> Self {
        let lines: Vec<String> = item.lines().collect::<Vec<&str>>().iter()
            .map(|x| x.trim().to_string()).collect();
        // monkey number
        let re_monkey_num = Regex::new(r"Monkey (\d+):$").unwrap();
        let caps_num = re_monkey_num.captures(&lines[0]).unwrap();
        let monkey_num = caps_num.get(1).unwrap().as_str()
            .parse::<MonkeyNumber>().unwrap();
        // starting items
        let (_, right) = lines[1].split_once(": ").unwrap();
        let starting_nums: Vec<WorryItem> = right.split(", ")
            .map(|x| x.parse::<WorryItem>().unwrap())
            .collect();
        // operation
        let (_, right) = lines[2].split_once("new = ").unwrap();
        let re_operation = Regex::new(r"(\w+) ([\+\*]) (\w+)").unwrap();
        let caps_operation = re_operation.captures(right).unwrap();
        let op_lh = caps_operation.get(1).unwrap()
            .as_str().to_string();
        let op_or = caps_operation.get(2).unwrap()
            .as_str().to_string();
        let op_rh = caps_operation.get(3).unwrap()
            .as_str().to_string();
        // println!("{:?} - {:?} - {:?}", op_lh, op_or, op_rh);
        let operation = op_creator(op_lh, op_or, op_rh);
        // test func
        let (_, right) = lines[3].split_once("divisible by ").unwrap();
        let condition = cond_creator(right.parse::<usize>().unwrap());
        // send_true
        let re_throw_to = Regex::new(r"throw to monkey (\d+)").unwrap();
        let caps_send_true = re_throw_to.captures(&lines[4]).unwrap();
        let send_true = caps_send_true.get(1).unwrap().as_str()
            .parse::<MonkeyNumber>().unwrap();
        // send_false
        let caps_send_false = re_throw_to.captures(&lines[5]).unwrap();
        let send_false = caps_send_false.get(1).unwrap().as_str()
            .parse::<MonkeyNumber>().unwrap();

        Monkey {
            number: monkey_num,
            inspect_count: 0,
            items: starting_nums,
            operation,
            condition,
            send_true,
            send_false,
        }
    }
}

impl Monkey {
    fn process_one_item(&self, item: WorryItem) -> (MonkeyNumber, WorryItem) {
        let after_op = (self.operation)(item);
        let after_divide = after_op / 3;
        match (self.condition)(after_divide) {
            true => (self.send_true, after_divide),
            false => (self.send_false, after_divide),
        }
    }

    fn process_items(&mut self) -> Vec<(MonkeyNumber, WorryItem)> {
        let mut results = vec![];
        for item in self.items.iter().cloned() {
            self.inspect_count += 1;
            let res = self.process_one_item(item);
            results.push(res);
        }
        // cannot drain and process_one_item?
        self.items = vec![];
        results
    } 
}

#[cfg(test)]
mod monkey_test {
    use super::*;

    const MONKEY_0: &str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3";

    #[test]
    fn it_can_parse_monkey_0() {
        let monkey_0_in = MONKEY_0.to_string();
        let expected = Monkey {
            number: 0,
            items: vec![79, 98],
            inspect_count: 0,
            operation: Box::new(    |old| old * 19),
            condition: Box::new(|x| x % 23 == 0),
            send_true: 2,
            send_false: 3,
        };
        let monkey = Monkey::from(monkey_0_in);
        assert_eq!(monkey, expected);
    }

    #[test]
    fn it_can_process_items() {
        let monkey_0_in = MONKEY_0.to_string();
        let mut monkey = Monkey::from(monkey_0_in);
        let results = monkey.process_items();
        let expected = vec![
            (3, 500),
            (3, 620),
        ];
        
        assert_eq!(monkey.items, vec![]);
        assert_eq!(results, expected);
    }
}