use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Not, Sub};
use crate::traits::{Dot, Cross, Norm, Sqrt, PieceWiseMul, PieceWiseDiv, Invert};

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec2<T> {
    x: T,
    y: T,
}

impl<T: Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }
}

impl<T: PartialOrd> Vec2<T> {
    pub fn all_smaller(&self, other: &Vec2<T>) -> bool {
        self.x < other.x && self.y < other.y
    }

    pub fn all_smaller_eq(&self, other: &Vec2<T>) -> bool {
        self.x <= other.x && self.y <= other.y
    }
}

impl<T> Vec2<T>
    where T: Copy + Mul<Output=T> + Add<Output=T> + Div<Output=T> + DivAssign + Neg<Output=T> + Sqrt {
    pub fn normal(&self) -> Vec2<T> {
        let normal = Vec2 { x: self.y, y: -self.x};
        normal / normal.norm()
    }
}

impl<T> Dot for Vec2<T>
    where T: Mul<Output=T> + Add<Output=T> + Copy {
    type Output = T;

    fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T> Cross for Vec2<T>
    where T: Mul<Output=T> + Sub<Output=T> + Copy {
    type Output = T;

    fn cross(&self, rhs: &Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl<T> Norm for Vec2<T>
    where T: Mul<Output=T> + Div<Output=T> + DivAssign + Add<Output=T> + Copy + Sqrt {
    type Output = T;

    fn norm(&self) -> Self::Output {
        T::sqrt(self.norm2())
    }

    fn norm2(&self) -> Self::Output {
        self.dot(self)
    }

    fn normalized(&self) -> Self {
        let norm = self.norm();
        Vec2 { x: self.x / norm, y: self.y / norm }
    }

    fn normalize(&mut self) {
        let norm = self.norm();
        self.x /= norm;
        self.y /= norm;
    }
}

impl<T> Add for Vec2<T>
    where T: Add<Output=T> + Copy {
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T> Sub for Vec2<T>
    where T: Sub<Output=T> + Copy {
    type Output = Vec2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T> AddAssign for Vec2<T> where T: AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Mul<T> for Vec2<T> where T: Mul<Output=T> + Copy {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl<T> Div<T> for Vec2<T> where T: Div<Output=T> + Copy {
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec2 { x: self.x / rhs, y: self.y / rhs }
    }
}

impl<T> PieceWiseMul for Vec2<T> where T: Mul<Output=T> + Copy {
    fn piece_wise_mul(&self, rhs: &Self) -> Self {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T> PieceWiseDiv for Vec2<T> where T: Div<Output=T> + Copy {
    fn piece_wise_div(&self, rhs: &Self) -> Self {
        Vec2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T> Invert for Vec2<T> where T: Neg<Output=T> + Copy {
    fn inverted(&self) -> Self {
        Vec2::new(-self.x, -self.y)
    }

    fn invert(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }
}

impl<T> Not for Vec2<T> where T: Not<Output=T> + Copy {
    type Output = Vec2<T>;

    fn not(self) -> Self::Output {
        Vec2::new(!self.x(), !self.y())
    }
}

impl<T> From<[T; 2]> for Vec2<T> where T: Copy {
    fn from(arr: [T; 2]) -> Self {
        Vec2::new(arr[0], arr[1])
    }
}