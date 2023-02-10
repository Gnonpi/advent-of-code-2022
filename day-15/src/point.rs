use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) struct Point {
    pub(crate) x: isize,
    pub(crate) y: isize,
}

impl From<String> for Point {
    fn from(item: String) -> Self {
        let re_capt_x_y = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
        let capt_x_y = re_capt_x_y.captures(&item).unwrap();
        let num_x = capt_x_y.get(1).unwrap().as_str().parse::<isize>().unwrap();
        let num_y = capt_x_y.get(2).unwrap().as_str().parse::<isize>().unwrap();
        Point { x: num_x, y: num_y }
    }
}

pub(crate) fn coverage_within_manhattan(p: &Point, dist: usize) -> HashSet<Point> {
    let mut result = HashSet::new();
    let idist = dist as isize;
    for diff_x in -idist..idist {
        for diff_y in -idist..idist {
            if diff_x.abs() + diff_y.abs() <= dist.try_into().unwrap() {
                result.insert(Point {
                    x: p.x + diff_x,
                    y: p.y + diff_y,
                });
            }
        }
    }
    result
}

pub(crate) fn coverage_within_manhattan_at_y(
    p: &Point,
    dist: usize,
    level_y: isize,
) -> HashSet<Point> {
    let mut result = HashSet::new();
    if manhattan_distance(p, &Point { x: p.x, y: level_y }) > dist {
        return result;
    }
    let idist = dist as isize;
    for diff_x in -idist..idist {
        let new_p = Point {
            x: p.x + diff_x,
            y: level_y,
        };
        if manhattan_distance(p, &new_p) <= dist {
            result.insert(new_p);
        }
    }
    result
}

pub(crate) fn manhattan_distance(p1: &Point, p2: &Point) -> usize {
    ((p1.x - p2.x).abs() + (p1.y - p2.y).abs())
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod point_test {
    use super::*;

    #[test]
    fn it_can_parse_x_y() {
        let line = String::from("x=-2, y=15");
        let expected = Point { x: -2, y: 15 };
        let p = Point::from(line);
        assert_eq!(p, expected);
    }

    #[test]
    fn it_can_compute_manhattan() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: -4, y: 6 };
        let expected = 9;
        assert_eq!(manhattan_distance(&p1, &p2), expected);
    }
}
