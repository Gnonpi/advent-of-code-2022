#[derive(Debug, PartialEq, Clone)]
pub(crate) enum CdDestination {
    Root,
    Back,
    Next(String),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct CdCommand {
    destination: CdDestination,
}

impl From<String> for CdCommand {
    fn from(input_string: String) -> Self {
        let (_, right) = input_string.split_once(" ").unwrap();
        match right {
            "/" => CdCommand {
                destination: CdDestination::Root,
            },
            ".." => CdCommand {
                destination: CdDestination::Back,
            },
            s => CdCommand {
                destination: CdDestination::Next(s.to_string()),
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum LsElementType {
    Directory,
    File,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct LsElement {
    ls_type: LsElementType,
    name: String,
    total_size: usize,
}

impl From<String> for LsElement {
    fn from(item: String) -> Self {
        let (left, right) = item.split_once(' ').unwrap();
        if left.starts_with("dir") {
            LsElement {
                ls_type: LsElementType::Directory,
                name: right.to_string(),
                total_size: 0,
            }
        } else {
            LsElement {
                ls_type: LsElementType::File,
                name: right.to_string(),
                total_size: left.parse::<usize>().unwrap(),
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct LsCommand {
    elements: Vec<LsElement>,
}

impl From<String> for LsCommand {
    fn from(item: String) -> Self {
        let mut elements = vec![];
        // reading elements without the header
        for line in item.split('\n').collect::<Vec<&str>>()[1..].iter() {
            elements.push(LsElement::from(line.to_string()))
        }
        LsCommand { elements }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Command {
    Cd(CdCommand),
    Ls(LsCommand),
}

impl From<String> for Command {
    fn from(item: String) -> Self {
        if item.starts_with("cd") {
            Command::Cd(CdCommand::from(item))
        } else if item.starts_with("ls") {
            Command::Ls(LsCommand::from(item))
        } else {
            panic!("unknown command: {:?}", item);
        }
    }
}

pub(crate) fn parse_commands(input_string: String) -> Vec<Command> {
    let mut result = vec![];
    for cmd_str in input_string.split('$').collect::<Vec<&str>>().iter() {
        let cmd_string = cmd_str.trim().to_string();
        if cmd_string.is_empty() {
            continue;
        }
        result.push(Command::from(cmd_string));
    }
    result
}

#[cfg(test)]
mod day_test {
    use super::*;

    #[test]
    fn it_can_parse_cds() {
        assert_eq!(
            CdCommand::from("cd /".to_string()),
            CdCommand {
                destination: CdDestination::Root
            }
        );
        assert_eq!(
            CdCommand::from("cd ..".to_string()),
            CdCommand {
                destination: CdDestination::Back
            }
        );
        assert_eq!(
            CdCommand::from("cd a".to_string()),
            CdCommand {
                destination: CdDestination::Next("a".to_string())
            }
        );
    }

    #[test]
    fn it_can_parse_ls() {
        let example = "ls
dir a
14848514 b.txt
8504156 c.dat
dir d"
            .to_string();
        let result = LsCommand::from(example);
        let expected = LsCommand {
            elements: vec![
                LsElement {
                    ls_type: LsElementType::Directory,
                    name: "a".to_string(),
                    total_size: 0,
                },
                LsElement {
                    ls_type: LsElementType::File,
                    name: "b.txt".to_string(),
                    total_size: 14848514,
                },
                LsElement {
                    ls_type: LsElementType::File,
                    name: "c.dat".to_string(),
                    total_size: 8504156,
                },
                LsElement {
                    ls_type: LsElementType::Directory,
                    name: "d".to_string(),
                    total_size: 0,
                },
            ],
        };
        assert_eq!(result, expected);
    }
}
