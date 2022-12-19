// not the best module name
use crate::movement::Movement;
use crate::point::Point;

pub(crate) fn apply_movement(point: &Point, mov: &Movement) -> Point {
    Point {
        x: point.x + mov.x,
        y: point.y + mov.y,
    }
}

pub(crate) fn is_head_adjacent(head: &Point, tail: &Point) -> bool {
    let sub = Movement {
        x: head.x - tail.x,
        y: head.y - tail.y,
    };
    (sub.x.abs() <= 1) && (sub.y.abs() <= 1)
}

pub(crate) fn follow_head(head: &Point, tail: &Point) -> Movement {
    if is_head_adjacent(head, tail) {
        return Movement { x: 0, y: 0 };
    }
    let sub = Movement {
        x: head.x - tail.x,
        y: head.y - tail.y,
    };
    sub.make_unary()
}
