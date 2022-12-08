use super::command::{
    Command, 
    CdDestination,
    LsElementType,
};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum FileType {
    Directory,
    File,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct File {
    file_type: FileType,
    pub(crate) name: String,
    pub(crate) total_size: usize,
    dirname: Option<String>,
}

impl File {
    fn full_path(&self) -> String {
        match self.dirname.clone() {
            None => self.name.clone(),
            Some(d) => {
                vec![d, self.name.clone()].join("/").replace("//", "/")
            }
        }    
    }
}

#[derive(Default, Debug, PartialEq)]
pub(crate) struct FileTree {
    files: HashMap<String, File>,
}

fn compare_slashes(a: &String, b: &String) -> std::cmp::Ordering {
    let a_l = a.matches("/").collect::<Vec<&str>>().len();
    let b_l = b.matches("/").collect::<Vec<&str>>().len();
    a_l.partial_cmp(&b_l).unwrap()
}

impl FileTree {
    fn register_file(&mut self, file: File) {
        self.files.insert(file.full_path(), file);
    }

    fn size_of_directories(&mut self) -> Vec<(String, usize)> {
        let mut sorted_abs_path = self.files.keys().cloned().collect::<Vec<String>>();
        sorted_abs_path.sort_by(|a, b| compare_slashes(a, b));
        sorted_abs_path.reverse();
        let mut result = vec![];
        // println!("{:?}", sorted_abs_path);
        for key in sorted_abs_path.iter() {
            let file = self.files.get(key).unwrap().clone();
            if file.name == "/".to_string() {
                continue
            }
            let dirname = file.dirname.unwrap().clone();
            let mut parent_path = dirname.trim_end_matches("/").to_string();
            if parent_path.is_empty() {
                parent_path = "/".to_string();
            }
            // println!("parent_path: {:?}", parent_path.clone());
            match file.file_type {
                FileType::File => {
                    if let Some(parent_dir) = self.files.get_mut(&parent_path) {
                        parent_dir.total_size += file.total_size;
                    }
                },
                FileType::Directory => {
                    if let Some(parent_dir) = self.files.get_mut(&parent_path) {
                        parent_dir.total_size += file.total_size;
                    }
                }
            }
        }
        result
    }

    pub(crate) fn get_directories(&self) -> Vec<File> {
        let mut keys: Vec<&String> = self.files.keys().clone().collect();
        keys.sort();
        let mut result = vec![];
        for key in keys {
            if let Some(file) = self.files.get(key) {
                match file.file_type {
                    FileType::Directory => {
                        result.push(file.clone())
                    },
                    _ => {
                        continue
                    }
                }
            }
        }
        result
    }
}

impl From<Vec<Command>> for FileTree {
    fn from(item: Vec<Command>) -> Self {
        let mut ft = FileTree::default();
        let mut current_pos = vec!["/".to_string()];
        ft.files.insert(
            "/".to_string(),
            File {
                name: "/".to_string(),
                file_type: FileType::Directory,
                total_size: 0,
                dirname: None
            }
        );
        for cmd in item.iter() {
            // println!("pos: {:?}", current_pos);
            match cmd {
                // handle moving around
                Command::Cd(cd) => {
                    match cd.destination.clone() {
                        CdDestination::Root => {
                            current_pos = vec!["/".to_string()];
                        },
                        CdDestination::Back => {
                            current_pos.pop();
                        },
                        CdDestination::Next(s) => {
                            current_pos.push(s + "/");
                        },
                    }
                },
                Command::Ls(ls) => {
                    for el in ls.elements.iter() {
                        let mut dirname = current_pos.join("/");
                        dirname = dirname.replace("//", "/");
                        // println!("{:?} -- {:?}", el.name, dirname);
                        match el.ls_type {
                            LsElementType::Directory => {
                                let file = File {
                                    name: el.name.to_string(),
                                    file_type: FileType::Directory,
                                    total_size: 0,
                                    dirname: Some(dirname.clone())
                                };
                                ft.register_file(file);
                            },
                            LsElementType::File => {
                                let dir = File {
                                    name: el.name.to_string(), 
                                    file_type: FileType::File,
                                    total_size: el.total_size,
                                    dirname: Some(dirname.clone())
                                };
                                ft.register_file(dir);
                            },
                        }
                    }
                }
            }
        }
        ft.size_of_directories();
        ft
    }
}


#[cfg(test)]
mod file_tree_test {
    use super::*;
    use super::super::command::parse_commands;

    #[test]
    fn it_can_build_example() {
        // example minus d
        let example = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..".to_string();
        let cmds = parse_commands(example);
        let ft = FileTree::from(cmds);
        let expected = FileTree {
            files: HashMap::from([
                ("/a/e/i".to_string(), File {
                    file_type: FileType::File,
                    name: "i".to_string(),
                    total_size: 584,
                    dirname: Some("/a/e/".to_string()),
                }),
                ("/a/g".to_string(), File {
                    file_type: FileType::File,
                    name: "g".to_string(),
                    total_size: 2557,
                    dirname: Some("/a/".to_string()),
                }),
                ("/a/e".to_string(), File {
                    file_type: FileType::Directory,
                    name: "e".to_string(),
                    total_size: 584,
                    dirname: Some("/a/".to_string()),
                }),
                ("/a/h.lst".to_string(), File {
                    file_type: FileType::File,
                    name: "h.lst".to_string(),
                    total_size: 62596,
                    dirname: Some("/a/".to_string()),
                }),
                ("/a/f".to_string(), File {
                    file_type: FileType::File,
                    name: "f".to_string(),
                    total_size: 29116,
                    dirname: Some("/a/".to_string()),
                }),
                ("/".to_string(), File {
                    file_type: FileType::Directory,
                    name: "/".to_string(),
                    total_size: 23447523,
                    dirname: None,
                }),
                ("/a".to_string(), File {
                    file_type: FileType::Directory,
                    name: "a".to_string(),
                    total_size: 94853,
                    dirname: Some("/".to_string()),
                }),
                ("/b.txt".to_string(), File {
                    file_type: FileType::File,
                    name: "b.txt".to_string(),
                    total_size: 14848514,
                    dirname: Some("/".to_string()),
                }),
                ("/c.dat".to_string(), File {
                    file_type: FileType::File,
                    name: "c.dat".to_string(),
                    total_size: 8504156,
                    dirname: Some("/".to_string()),
                })
            ])
        };
        // println!("ft: {:?}", ft);
        // println!("keys: {:?}", ft.files.keys());
        // println!("sizes: {:?}", ft.size_of_directories());

        let mut keys: Vec<&String> = ft.files.keys().clone().collect();
        keys.sort();
        for key in keys {
            println!("Comparing: {:?}", key);
            let f = ft.files.get(key).unwrap();
            let e = expected.files.get(key).unwrap();
            assert_eq!(f, e);
        }
        
        assert_eq!(ft, expected);
    }
}