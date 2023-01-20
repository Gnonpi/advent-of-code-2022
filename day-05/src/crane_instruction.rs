#[derive(Debug, PartialEq, Clone)]
pub(crate) struct CraneInstruction {
    pub(crate) quantity: u32,
    pub(crate) from: u32,
    pub(crate) to: u32,
}

impl From<String> for CraneInstruction {
    fn from(input_string: String) -> Self {
        let split: Vec<String> = input_string.split_whitespace().map(|x| { x.to_string() }).collect();
        CraneInstruction {
            quantity: split[1].parse::<u32>().unwrap(),
            from: split[3].parse::<u32>().unwrap(),
            to: split[5].parse::<u32>().unwrap(),
        }
    }
}

#[cfg(test)]
mod day_test {
    use super::*;

    const EXAMPLE: &str = "move 3 from 2 to 1";

    #[test]
    fn it_can_parse_instruction() {
        let input_string  = String::from(EXAMPLE);
        let expected = CraneInstruction {
            quantity: 3,
            from: 2,
            to: 1
        };
        let result = CraneInstruction::from(input_string);
        assert_eq!(expected, result);
    }
}