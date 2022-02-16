pub trait Dot<Rhs = Self> {
    type Output;
    fn dot(&self, rhs: &Rhs) -> Self::Output;
}

pub trait Cross<Rhs = Self> {
    type Output;
    fn cross(&self, rhs: &Rhs) -> Self::Output;
}

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
}

pub trait Norm {
    type Output;
    fn norm(&self) -> Self::Output;
    fn norm2(&self) -> Self::Output;
    fn normalized(&self) -> Self;
    fn normalize(&mut self);
}

pub trait PieceWiseMul {
    fn piece_wise_mul(&self, rhs: &Self) -> Self;
}

pub trait PieceWiseDiv {
    fn piece_wise_div(&self, rhs: &Self) -> Self;
}

pub trait Invert {
    fn inverted(&self) -> Self;
    fn invert(&mut self);
}