use crate::move_func::{apply_movement, follow_head};
use crate::movement::Movement;
use crate::point::Point;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Rope {
    nb_nodes: usize,
    nodes: Vec<Point>,
    tail_memory: HashSet<Point>,
}

impl Rope {
    pub(crate) fn new(start: Point, length: usize) -> Self {
        Rope {
            nb_nodes: length,
            nodes: vec![start; length],
            tail_memory: HashSet::new(),
        }
    }

    pub(crate) fn apply_movement(&mut self, mov: Movement) {
        let unaries = mov.split_unit();
        for unit_mov in unaries {
            // move head,
            // update node after if needed
            // register tail position
            self.nodes[0] = apply_movement(&self.nodes[0], &unit_mov);
            for i in 0..self.nb_nodes {
                if i == 0 {
                    // skip head
                    continue;
                }
                // next node follow previous
                let node_mov = follow_head(&self.nodes[i - 1], &self.nodes[i]);
                self.nodes[i] = apply_movement(&self.nodes[i], &node_mov);
            }
            self.tail_memory
                .insert(self.nodes[self.nb_nodes - 1].clone());
        }
    }

    pub(crate) fn get_tail_memory(&self) -> HashSet<Point> {
        self.tail_memory.clone()
    }
}

#[cfg(test)]
mod rope_test {
    use super::*;

    const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    // rope with length of two should solve first part
    fn it_can_solve_first_part() {
        let mut parsed = vec![];
        for line in EXAMPLE.to_string().lines() {
            if line.is_empty() {
                continue;
            }
            parsed.push(Movement::from(line.trim().to_string()));
        }

        let rope_2 = Rope::new(Point::new(0, 0), 2);

        let tail_mem = rope_2.get_tail_memory();

        assert_eq!(tail_mem.len(), 13);
    }
}
