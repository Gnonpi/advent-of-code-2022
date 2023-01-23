use super::crane_instruction::CraneInstruction;

#[derive(Debug, PartialEq, Default, Clone)]
pub(crate) struct WarehouseState<T> {
    piles: Vec<PileOfCrate<T>>
}

impl<T: Copy + Clone> WarehouseState<T> {
    pub(crate) fn get_tops(&self) -> Vec<T> {
        let mut result = vec![];
        for p in self.piles.iter() {
            result.push(*p.top().unwrap());
        }
        result
    }

    pub(crate) fn execute_command(&mut self, instr: CraneInstruction) {
        let idx_from = instr.from - 1;
        let idx_to = instr.to - 1;
        for _ in 0..instr.quantity {
            let el = self.piles[idx_from as usize].pull_top();
            self.piles[idx_to as usize].add_on_top(el);
        }
    }

    pub(crate) fn execute_command_9001(&mut self, instr: CraneInstruction) {
        let idx_from = instr.from - 1;
        let idx_to = instr.to - 1;
        let pulled: Vec<T> = self.piles[idx_from as usize]
            .pull_n(instr.quantity).into_iter().collect();
        for el in pulled {
            self.piles[idx_to as usize].add_on_top(el);
        }
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
struct PileOfCrate<T> {
    pile: Vec<T>
}

impl<T: Clone> PileOfCrate<T> {
    fn add_on_top(&mut self, el: T) {
        self.pile.push(el);
    }

    fn add_on_bottom(&mut self, el: T) {
        self.pile.insert(0, el)
    }

    fn pull_top(&mut self) -> T {
        self.pile.pop().unwrap()    
    }

    fn top(&self) -> Option<&T> {
        self.pile.last()
    }

    fn pull_n(&mut self, n: u32) -> Vec<T> {
        let binding = self.pile.clone();
        let (state, pulled) = binding.split_at(self.pile.len() - (n as usize));
        self.pile = state.to_vec();
        pulled.to_vec()
    }
}

impl From<String> for WarehouseState<char> {
    fn from(input_string: String) -> Self {
        let split_crane: Vec<String> = input_string.split('\n').filter_map(|x| { 
            let c = x.to_string();
            if c.is_empty() {
                None
            } else {
                Some(c)    
            }
        }).collect();
        // find how many piles are there
        let nb_piles = split_crane.last().unwrap().split_whitespace().last().unwrap().parse::<u32>().unwrap();
        // println!("Got {:?} piles", nb_piles);
        // adding an empty vec for each location
        let mut initial: WarehouseState<char> = WarehouseState::default();
        for _ in 0..nb_piles {
            initial.piles.push(PileOfCrate::default());
        }
        for line in split_crane.iter() {
            if line.starts_with(" 1") {
                break
            }
            // println!("w line: {:?}", line);
            for (i, crate_content) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                if crate_content.is_empty() {
                    continue
                }
                // println!("i: {:?}", i);
                let mut letter = None;
                // functional was getting super ugly
                for l in crate_content.iter().cloned() {
                    if l != ' ' && l != '[' && l != ']' {
                        letter = Some(l);
                        break
                    }
                }
                // println!("content: {:?}", letter);
                match letter {
                    Some(c) => {
                        initial.piles[i].add_on_bottom(c);
                    }
                    None => {}
                }
            }
        }
        initial
    }
}

#[cfg(test)]
mod day_test {
    use super::*;

    const EXAMPLE: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
";

    #[test]
    fn it_can_parse_warehouse_state() {
        let parsed = WarehouseState::from(EXAMPLE.to_string());
        let expected = WarehouseState {
            piles: vec![
                PileOfCrate { pile: vec!['Z', 'N'] },
                PileOfCrate { pile: vec!['M', 'C', 'D'] },
                PileOfCrate { pile: vec!['P'] },
            ]
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn it_can_execute_command() {
        let mut initial = WarehouseState {
            piles: vec![
                PileOfCrate { pile: vec!['Z', 'N'] },
                PileOfCrate { pile: vec!['M', 'C', 'D'] },
                PileOfCrate { pile: vec!['P'] },
            ]
        };
        initial.execute_command(CraneInstruction {
            quantity: 1, 
            from: 2,
            to: 1
        });
        initial.execute_command(CraneInstruction {
            quantity: 3, 
            from: 1,
            to: 3
        });
        let expected = WarehouseState {
            piles: vec![
                PileOfCrate { pile: vec![] },
                PileOfCrate { pile: vec!['M', 'C'] },
                PileOfCrate { pile: vec!['P', 'D', 'N', 'Z'] },
            ]
        };
        assert_eq!(initial, expected);
    }

    #[test]
    fn it_can_pull_n() {
        let mut p = PileOfCrate { pile: vec![1, 2 ,3] };
        let pulled = p.pull_n(2);
        assert_eq!(p, PileOfCrate { pile: vec![1] });
        assert_eq!(pulled, vec![2, 3]);
    }

    #[test]
    fn it_can_execute_command_9001() {
        let mut initial = WarehouseState {
            piles: vec![
                PileOfCrate { pile: vec!['Z', 'N'] },
                PileOfCrate { pile: vec!['M', 'C', 'D'] },
                PileOfCrate { pile: vec!['P'] },
            ]
        };
        initial.execute_command_9001(CraneInstruction {
            quantity: 1, 
            from: 2,
            to: 1
        });
        initial.execute_command_9001(CraneInstruction {
            quantity: 3, 
            from: 1,
            to: 3
        });
        let expected = WarehouseState {
            piles: vec![
                PileOfCrate { pile: vec![] },
                PileOfCrate { pile: vec!['M', 'C'] },
                PileOfCrate { pile: vec!['P', 'Z', 'N', 'D'] },
            ]
        };
        assert_eq!(initial, expected);
    }
}