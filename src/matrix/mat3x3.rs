use std::ops::{Add, Index, Mul};
use std::process::Output;
use crate::traits::Dot;
use crate::vector::Vec3;

#[derive(Debug, Default)]
pub struct Mat3x3<T> {
    values: [[T; 3]; 3]
}

impl<T: Copy> Mat3x3<T> {
    pub fn new(values: [[T; 3]; 3]) -> Self {
        Mat3x3 { values }
    }

    pub fn row(&self, index: usize) -> Vec3<T> {
        Vec3::from(self[index])
    }

    pub fn col(&self, index: usize) -> Vec3<T> {
        Vec3::new(
            self[0][index],
            self[1][index],
            self[2][index],
        )
    }
}

impl<T> Index<usize> for Mat3x3<T> {
    type Output = [T; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl<T: Mul<Output=T> + Add<Output=T> + Copy> Mat3x3<T> {
    pub fn dot(&self, rhs: &Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.row(0).dot(rhs),
            self.row(1).dot(rhs),
            self.row(2).dot(rhs)
        )
    }
}