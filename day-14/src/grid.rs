use super::point::Point;
use std::cmp::{max, min};
use std::collections::HashSet;

const MAX_DEPTH: usize = 500;
const FLOOR_OFFSET: usize = 2;

#[derive(PartialEq, Clone, Default)]
pub(crate) struct FallingGrid {
    filled: HashSet<Point>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) enum GrainStatus {
    Stopped,
    Falling,
}

impl FallingGrid {
    pub(crate) fn add_line(&mut self, start: Point, end: Point) {
        let range_x = min(start.x, end.x)..max(start.x, end.x);
        let range_y = min(start.y, end.y)..max(start.y, end.y);

        let mut xs: Vec<usize> = range_x.collect();
        let mut ys: Vec<usize> = range_y.collect();
        if xs.is_empty() {
            xs = vec![start.x];
        }
        if ys.is_empty() {
            ys = vec![start.y];
        }

        for x in xs.iter() {
            for y in ys.iter() {
                self.add_fixed_point(Point { x: *x, y: *y });
            }
        }
        self.add_fixed_point(end);
    }

    fn add_fixed_point(&mut self, p: Point) {
        self.filled.insert(p);
    }

    pub(crate) fn set_floor(&mut self) {
        // find lowest point
        let lowest_y = self
            .filled
            .iter()
            .map(|p| p.y)
            .reduce(max)
            .unwrap();
        // add thousand of floor points
        for x in 0..1000 {
            self.add_fixed_point(Point {
                x,
                y: lowest_y + FLOOR_OFFSET,
            })
        }
    }

    fn get_number_of_points(&self) -> usize {
        self.filled.len()
    }

    pub(crate) fn fall_one_sand(&mut self, start: Point) -> GrainStatus {
        let mut current = start;
        let mut status = GrainStatus::Falling;
        while current.y < MAX_DEPTH && status == GrainStatus::Falling {
            match self.are_under_free(&current) {
                (_, true, _) => {
                    current.y += 1;
                }
                (true, false, _) => {
                    current.y += 1;
                    current.x -= 1;
                }
                (false, false, true) => {
                    current.y += 1;
                    current.x += 1;
                }
                (false, false, false) => {
                    self.add_fixed_point(current);
                    status = GrainStatus::Stopped;
                }
            }
        }
        status
    }

    fn are_under_free(&self, p: &Point) -> (bool, bool, bool) {
        let mut p_x_left = p.x;
        if p.x == 0 {
            p_x_left = 0;
        } else {
            p_x_left = p.x - 1;
        }
        let p_diag_left = Point {
            x: p_x_left,
            y: p.y + 1,
        };
        let p_directly_under = Point { x: p.x, y: p.y + 1 };
        let p_diag_right = Point {
            x: p.x + 1,
            y: p.y + 1,
        };
        (
            // diagonal left
            self.is_point_free(&p_diag_left),
            // directly under
            self.is_point_free(&p_directly_under),
            // diagonal right
            self.is_point_free(&p_diag_right),
        )
    }

    pub(crate) fn is_point_free(&self, p: &Point) -> bool {
        !self.filled.contains(p)
    }

    pub(crate) fn is_point_occupied(&self, p: &Point) -> bool {
        self.filled.contains(p)
    }
}

#[cfg(test)]
mod falling_grid_test {
    use super::*;

    #[test]
    fn it_can_add_point() {
        let mut fg = FallingGrid::default();
        assert_eq!(fg.get_number_of_points(), 0);
        let p = Point { x: 0, y: 0 };
        fg.add_fixed_point(p);
        assert_eq!(fg.get_number_of_points(), 1);
        assert_eq!(fg.is_point_free(&p), false);
        assert!(fg.is_point_occupied(&p));
    }

    #[test]
    fn it_can_create_lines() {
        let mut fg = FallingGrid::default();
        let start = Point { x: 0, y: 0 };
        let end = Point { x: 3, y: 0 };
        fg.add_line(start, end);
        assert_eq!(fg.get_number_of_points(), 4);
        assert!(fg.is_point_occupied(&Point { x: 0, y: 0 }));
        assert!(fg.is_point_occupied(&Point { x: 1, y: 0 }));
        assert!(fg.is_point_occupied(&Point { x: 2, y: 0 }));
        assert!(fg.is_point_occupied(&Point { x: 3, y: 0 }));
    }

    #[test]
    fn it_can_fall_one_grain() {
        let mut fg = FallingGrid::default();
        let p0 = Point { x: 10, y: 1 };
        let p1 = Point { x: 11, y: 1 };
        let p2 = Point { x: 12, y: 1 };
        fg.add_fixed_point(p0);
        fg.add_fixed_point(p1);
        fg.add_fixed_point(p2);
        assert_eq!(fg.get_number_of_points(), 3);

        // grain that stops
        let start = Point { x: 11, y: 0 };
        let r = fg.fall_one_sand(start);
        assert_eq!(r, GrainStatus::Stopped);
        assert_eq!(fg.get_number_of_points(), 4);

        // grain that felloff
        let start = Point { x: 15, y: 0 };
        let r = fg.fall_one_sand(start);
        assert_eq!(r, GrainStatus::Falling);
        assert_eq!(fg.get_number_of_points(), 4);
    }

    #[test]
    fn it_can_check_free_point() {
        let mut fg = FallingGrid::default();
        let p0 = Point { x: 0, y: 0 };
        let p1 = Point { x: 1, y: 0 };
        fg.add_fixed_point(p0);
        assert!(!fg.is_point_free(&p0));
        assert!(fg.is_point_free(&p1));
        assert!(fg.is_point_occupied(&p0));
        assert!(!fg.is_point_occupied(&p1));

        fg.add_fixed_point(p0);
        assert!(!fg.is_point_free(&p0));
        assert!(fg.is_point_occupied(&p0));
    }

    #[test]
    fn it_can_check_are_under_free() {
        let mut fg = FallingGrid::default();
        let p0 = Point { x: 0, y: 1 };
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 1 };
        fg.add_fixed_point(p0);
        fg.add_fixed_point(p1);
        fg.add_fixed_point(p2);

        assert_eq!(fg.are_under_free(&Point { x: 0, y: 2 }), (true, true, true));
        assert_eq!(
            fg.are_under_free(&Point { x: 3, y: 0 }),
            (false, true, true)
        );
        assert_eq!(
            fg.are_under_free(&Point { x: 2, y: 0 }),
            (false, false, true)
        );
        assert_eq!(
            fg.are_under_free(&Point { x: 1, y: 0 }),
            (false, false, false)
        );
    }

    #[test]
    fn it_can_add_floor() {
        let mut fg = FallingGrid::default();
        let low_y = 3;
        let p0 = Point { x: 0, y: low_y };
        fg.add_fixed_point(p0);
        fg.set_floor();
        let nb_pts = fg.get_number_of_points();
        assert!(nb_pts > 3);
        assert!(fg.is_point_occupied(&Point {
            x: 0,
            y: low_y + FLOOR_OFFSET
        }));
        assert!(fg.is_point_occupied(&Point {
            x: 100,
            y: low_y + FLOOR_OFFSET
        }));
    }
}
