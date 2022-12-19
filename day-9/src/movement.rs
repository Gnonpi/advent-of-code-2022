#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Movement {
    pub(crate) x: isize,
    pub(crate) y: isize,
}

impl From<String> for Movement {
    fn from(item: String) -> Self {
        let (left, right) = item.split_once(' ').unwrap();
        let mut x = 0;
        let mut y = 0;
        let right_num = right.parse::<isize>().unwrap();
        match left {
            "L" => {
                x = -right_num;
            }
            "R" => {
                x = right_num;
            }
            "U" => {
                y = right_num;
            }
            "D" => {
                y = -right_num;
            }
            _ => panic!("unknown letter: {:?}", left),
        }
        Movement { x, y }
    }
}

impl Movement {
    pub(crate) fn new(x: isize, y: isize) -> Self {
        Movement { x, y }
    }

    pub(crate) fn make_unary(&self) -> Movement {
        let xnum: usize = self.x.abs().try_into().unwrap();
        let ynum: usize = self.y.abs().try_into().unwrap();
        let mut new_x = 0;
        if xnum > 0 {
            new_x = self.x / (xnum as isize);
        }
        let mut new_y = 0;
        if ynum > 0 {
            new_y = self.y / (ynum as isize);
        }
        Movement { x: new_x, y: new_y }
    }

    pub(crate) fn split_unit(&self) -> Vec<Movement> {
        let unary = self.make_unary();
        if self.x != 0 {
            let num: usize = self.x.abs().try_into().unwrap();
            vec![unary; num]
        } else {
            let num: usize = self.y.abs().try_into().unwrap();
            vec![unary; num]
        }
    }
}
