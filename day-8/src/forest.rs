#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up, 
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub(crate) struct Tree {
    value: u8,
    visible_up: bool,
    visible_down: bool,
    visible_left: bool,
    visible_right: bool,
}

impl Tree {
    fn new(value: u8) -> Self {
        let mut tree = Tree::default();
        tree.value = value;
        tree
    }

    fn update_visibility(&mut self, is_visible: bool, direction: Direction) {
        match direction {
            Direction::Up => {
                self.visible_up = is_visible;
            },
            Direction::Down => {
                self.visible_down = is_visible;
            },
            Direction::Left => {
                self.visible_left = is_visible;
            },
            Direction::Right => {
                self.visible_right = is_visible;
            },
        }
    }
    
    pub(crate) fn is_any_visible(&self) -> bool {
        self.visible_up || self.visible_down || self.visible_left || self.visible_right
    }

    pub(crate) fn is_all_visible(&self) -> bool {
        self.visible_up && self.visible_down && self.visible_left && self.visible_right
    }
}

fn visible_trees(trees: &[Tree]) -> Vec<bool> {
    let mut previous = 0;
    let mut result = vec![];
    for t in trees.iter() {
        if t.value == 0 && previous == 0 {
            result.push(true);
            continue
        }
        if t.value > previous {
            previous = t.value;
            result.push(true);
        } else {
            result.push(false);
        }
    }
    result
}

#[derive(Debug, PartialEq, Clone, Default)]
pub(crate) struct Forest {
    trees: Vec<Vec<Tree>>
}

impl From<Vec<Vec<u32>>> for Forest {
    fn from(item: Vec<Vec<u32>>) -> Self {
        let mut forest = Forest::default();
        for row in item {
            let mut row_tree = vec![];
            for c in row {
                row_tree.push(Tree::new(c as u8));
            }
            forest.trees.push(row_tree);
        }
        forest
    }
}

impl Forest {
    pub(crate) fn get_size(&self) -> (usize, usize) {
        (self.trees.len(), self.trees[0].len())
    }

    fn get_trees(&self, n: usize, from: Direction) -> Vec<Tree> {
        match from {
            Direction::Up => {
                let mut buf = vec![];
                for row in self.trees.iter() {
                    buf.push(row[n].clone());
                }
                buf
            },
            Direction::Down => {
                let mut buf = vec![];
                for row in self.trees.iter() {
                    buf.push(row[n].clone());
                }
                buf.reverse();
                buf
            },
            Direction::Left => {
                self.trees[n].clone()
            },
            Direction::Right => {
                let mut row = self.trees[n].clone();
                row.reverse();
                row
            },
        }
    }

    fn get_trees_idx(&self, n: usize, from: Direction) -> Vec<(usize, usize)> {
        match from {
            Direction::Up => {
                let mut buf = vec![];
                for (i, _) in self.trees.iter().enumerate() {
                    buf.push((i, n));
                }
                buf
            },
            Direction::Down => {
                let mut buf = vec![];
                for (i, _) in self.trees.iter().enumerate() {
                    buf.push((i, n));
                }
                buf.reverse();
                buf
            },
            Direction::Left => {
                let buf = self.trees[n].iter().enumerate().map(|(i, _)| (n, i)).collect();
                buf
            },
            Direction::Right => {
                let mut buf: Vec<(usize, usize)> = self.trees[n].iter().enumerate().map(|(i, _)| (n, i)).collect();
                buf.reverse();
                buf
            },
        }
    }

    fn update_visibility_direction(&mut self, direction: Direction) {
        let mut limit = 0;
        match direction {
            Direction::Up|Direction::Down => { limit = self.trees.len() },
            Direction::Left|Direction::Right => { limit = self.trees[0].len() }
        }
        for i in 0..limit {
            // let mut mrow = self.get_trees_mut(i, direction);
            let row = self.get_trees(i, direction);
            let t_idx = self.get_trees_idx(i, direction);
            let vec_vis = visible_trees(&row);
            for (j, vis) in vec_vis.iter().enumerate() {
                let (l, u) = t_idx[j];
                // if l == 1 && u == 2 {
                //     println!("{:?}->{:?}", direction, vis);
                // }
                self.trees[l][u].update_visibility(*vis, direction);
            }
        }
    }

    pub(crate) fn update_visibility(&mut self) {
        self.update_visibility_direction(Direction::Up);
        self.update_visibility_direction(Direction::Down);
        self.update_visibility_direction(Direction::Left);
        self.update_visibility_direction(Direction::Right);
    }

    pub(crate) fn get_trees_flatten(&self) -> Vec<Tree> {
        self.trees.iter().flatten().cloned().collect()
    }
}


#[cfg(test)]
mod forest_test {
    use super::*;

    #[test]
    fn it_can_get_trees_from_direction() {
        let parsed: Vec<Vec<u32>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let forest = Forest::from(parsed);
        let first_left = forest.get_trees(0, Direction::Left);
        let expected: Vec<Tree> = vec![3, 0, 3, 7, 3].iter().map(|x| Tree::new(*x)).collect();
        assert_eq!(first_left, expected);
     
        let last_right = forest.get_trees(4, Direction::Right);
        let expected: Vec<Tree> = vec![0, 9, 3, 5, 3].iter().map(|x| Tree::new(*x)).collect();
        assert_eq!(last_right, expected);

        let one_up = forest.get_trees(1, Direction::Up);
        let expected: Vec<Tree> = vec![0, 5, 5, 3, 5].iter().map(|x| Tree::new(*x)).collect();
        assert_eq!(one_up, expected);

        let penul_down = forest.get_trees(3, Direction::Down);
        let expected: Vec<Tree> = vec![9, 4, 3, 1, 7].iter().map(|x| Tree::new(*x)).collect();
        assert_eq!(penul_down, expected);
    }

    #[test]
    fn it_can_find_visible() {
        let parsed: Vec<Vec<u32>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let forest = Forest::from(parsed);
        let row = forest.get_trees(0, Direction::Left);
        let visible = visible_trees(&row);
        let expected = vec![true, false, false, true, false];
        assert_eq!(visible, expected);
        
        let row = forest.get_trees(3, Direction::Up);
        let visible = visible_trees(&row);
        let expected = vec![true, false, false, false, true];
        assert_eq!(visible, expected);
    }

    #[test]
    fn it_can_update_visibility() {
        let parsed: Vec<Vec<u32>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let mut forest = Forest::from(parsed);
        forest.update_visibility();

        let left_middle_5 = forest.trees[2][1].clone();
        let expected = Tree {
            value: 5,
            visible_up: false,
            visible_down: false,
            visible_left: false,
            visible_right: true,
        };
        assert_eq!(left_middle_5, expected);

        let top_left_5 = forest.trees[1][1].clone();
        let expected = Tree {
            value: 5,
            visible_up: true,
            visible_down: false,
            visible_left: true,
            visible_right: false,
        };
        assert_eq!(top_left_5, expected);

        let center_3 = forest.trees[2][2].clone();
        let expected = Tree {
            value: 3,
            visible_up: false,
            visible_down: false,
            visible_left: false,
            visible_right: false,
        };
        assert_eq!(center_3, expected);

        let right_middle_3 = forest.trees[2][3].clone();
        let expected = Tree {
            value: 3,
            visible_up: false,
            visible_down: false,
            visible_left: false,
            visible_right: true,
        };
        assert_eq!(right_middle_3, expected);
    }

    #[test]
    fn it_can_instantiate_forest() {
        let parsed: Vec<Vec<u32>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let expected = Forest {
            trees: vec![
                vec![Tree::new(3), Tree::new(0), Tree::new(3), Tree::new(7), Tree::new(3)],
                vec![Tree::new(2), Tree::new(5), Tree::new(5), Tree::new(1), Tree::new(2)],
                vec![Tree::new(6), Tree::new(5), Tree::new(3), Tree::new(3), Tree::new(2)],
                vec![Tree::new(3), Tree::new(3), Tree::new(5), Tree::new(4), Tree::new(9)],
                vec![Tree::new(3), Tree::new(5), Tree::new(3), Tree::new(9), Tree::new(0)],
            ]
        };
        let result = Forest::from(parsed);
        assert_eq!(result, expected);
    }
}