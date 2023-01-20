#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) struct Point {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl From<String> for Point {
    fn from(item: String) -> Self {
        let (left, right) = item.trim().split_once(',').unwrap();
        Point {
            x: left.parse::<usize>().unwrap(),
            y: right.parse::<usize>().unwrap(),
        }
    }
}

#[cfg(test)]
mod point_test {
    use super::*;

    #[test]
    fn it_can_parse_tuple() {
        let s = String::from("496,4");
        let p = Point::from(s);
        let expected = Point { x: 496, y: 4 };
        assert_eq!(p, expected);
    }
}
