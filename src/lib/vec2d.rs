use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Vec2D(isize, isize);
pub const DIRECTIONS_8: [Vec2D; 8] = [
    Vec2D(0, -1),
    Vec2D(1, 0),
    Vec2D(0, 1),
    Vec2D(-1, 0),
    Vec2D(-1, -1),
    Vec2D(1, 1),
    Vec2D(-1, 1),
    Vec2D(1, -1),
];

impl Vec2D {
    pub fn new(x: isize, y: isize) -> Self {
        Vec2D(x, y)
    }
    pub fn x(&self) -> isize {
        self.0
    }
    pub fn y(&self) -> isize {
        self.1
    }
}

impl FromIterator<isize> for Vec2D {
    fn from_iter<T: IntoIterator<Item = isize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Vec2D(
            iter.next().expect("Expecting x coord"),
            iter.next().expect("Expecting y coord")
        )
    }
}

pub const DIRECTIONS: [Vec2D; 4] = [
    Vec2D(0, -1),
    Vec2D(1, 0),
    Vec2D(0, 1),
    Vec2D(-1, 0),
];

impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Vec2D {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2D(self.0 - rhs.0, self.1 - rhs.1)
    }
}
