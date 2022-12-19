#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Point {
    pub(crate) x: isize,
    pub(crate) y: isize,
}

impl Point {
    pub(crate) fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}
