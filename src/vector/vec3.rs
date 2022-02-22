use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, Neg, Not, Sub};
use crate::traits::{Cross, Dot, Invert, Norm, PieceWiseDiv, PieceWiseMul, Sqrt};

#[derive(Debug, Default, Copy, Clone)]
pub struct Vec3<T> {
    pub(crate) values: [T; 3]
}

impl<T: Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { values: [x, y, z] }
    }

    pub fn x(&self) -> T {
        self[0]
    }

    pub fn y(&self) -> T {
        self[1]
    }

    pub fn z(&self) -> T {
        self[2]
    }
}

impl<T: PartialOrd + Copy> Vec3<T> {
    pub fn all_smaller(&self, other: &Vec3<T>) -> bool {
        self.values[0] < other.values[0] && self.values[1] < other.values[1] && self.values[2] < other.values[2]
    }

    pub fn all_smaller_eq(&self, other: &Vec3<T>) -> bool {
        self.values[0] <= other.values[0] && self.values[1] <= other.values[1] && self.values[2] <= other.values[2]
    }

    pub fn min(&self) -> T {
        if self.values[0] <= self.values[1] && self.values[0] <= self.values[2] {
            self.values[0]
        } else if self.values[1] <= self.values[0] && self.values[1] <= self.values[2] {
            self.values[1]
        } else {
            self.values[2]
        }
    }

    pub fn max(&self) -> T {
        if self.values[0] >= self.values[1] && self.values[0] >= self.values[2] {
            self.values[0]
        } else if self.values[1] >= self.values[0] && self.values[1] >= self.values[2] {
            self.values[1]
        } else {
            self.values[2]
        }
    }

    pub fn element_wise_min(&self, other: &Self) -> Self {
        let x = if self.values[0] < other.values[0] { self.values[0] } else { other.values[0] };
        let y = if self.values[1] < other.values[1] { self.values[1] } else { other.values[1] };
        let z = if self.values[2] < other.values[2] { self.values[2] } else { other.values[2] };
        Vec3::new(x, y, z)
    }

    pub fn element_wise_max(&self, other: &Self) -> Self {
        let x = if self.values[0] >= other.values[0] { self.values[0] } else { other.values[0] };
        let y = if self.values[1] >= other.values[1] { self.values[1] } else { other.values[1] };
        let z = if self.values[2] >= other.values[2] { self.values[2] } else { other.values[2] };
        Vec3::new(x, y, z)
    }
}

impl<T> From<[T; 3]> for Vec3<T> {
    fn from(arr: [T; 3]) -> Self {
        Vec3 { values: arr }
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl<T> Vec3<T>
    where T: Mul<T, Output=T> + Add<T, Output=T> + Copy {
    pub fn piece_wise_mul(&self, rhs: &Self) -> Self {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl<T> Dot for Vec3<T>
    where T: Mul<Output=T> + Add<Output=T> + Copy {
    type Output = T;

    fn dot(&self, rhs: &Self) -> T {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
}

impl<T> Cross for Vec3<T>
    where T: Mul<Output=T> + Sub<Output=T> + Copy {
    type Output = Self;

    fn cross(&self, rhs: &Self) -> Self::Output {
        Vec3::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x()
        )
    }
}

impl<T> Norm for Vec3<T>
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
        Vec3::new(self.x() / norm, self.y() / norm, self.z() / norm)
    }

    fn normalize(&mut self) {
        let norm = self.norm();
        self[0] /= norm;
        self[1] /= norm;
        self[2] /= norm;
    }
}

impl<T> Add for Vec3<T>
    where T: Add<Output=T> + Copy {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z()
        )
    }
}

impl<T> Sub for Vec3<T>
    where T: Sub<Output=T> + Copy {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z()
        )
    }
}

impl<T> AddAssign for Vec3<T> where T: AddAssign + Copy {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs.x();
        self[1] += rhs.y();
        self[2] += rhs.z();
    }
}

impl<T> Mul<T> for Vec3<T> where T: Mul<Output=T> + Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3::new(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs
        )
    }
}

impl<T> Div<T> for Vec3<T> where T: Div<Output=T> + Copy {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vec3::new(
            self.x() / rhs,
            self.y() / rhs,
            self.z() / rhs
        )
    }
}

impl<T> PieceWiseMul for Vec3<T> where T: Mul<Output=T> + Copy {
    fn piece_wise_mul(&self, rhs: &Self) -> Self {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl<T> PieceWiseDiv for Vec3<T> where T: Div<Output=T> + Copy {
    fn piece_wise_div(&self, rhs: &Self) -> Self {
        Vec3::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
    }
}

impl<T> Neg for Vec3<T> where T: Neg<Output=T> + Copy {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl<T> Invert for Vec3<T> where T: Neg<Output=T> + Copy {
    fn inverted(&self) -> Self {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }

    fn invert(&mut self) {
        self[0] = -self.x();
        self[1] = -self.y();
        self[2] = -self.z();
    }
}

impl<T> Not for Vec3<T> where T: Not<Output=T> + Copy {
    type Output = Vec3<T>;

    fn not(self) -> Self::Output {
        Vec3::new(!self.x(), !self.y(), !self.z())
    }
}