mod vector;
mod traits;

#[cfg(test)]
mod tests;
mod matrix;

pub mod prelude {
    pub use super::traits::{Dot, Cross, Norm, Invert};
    pub use super::vector::{Vec2, Vec3};
    pub use super::matrix::Mat3x3;
}