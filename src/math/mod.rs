mod vector2;
mod matrix4x4;
mod rectangle;

pub use vector2::Vector2;
pub use matrix4x4::Matrix4x4;
pub use rectangle::Rectangle;

use std::ops::{AddAssign, Mul};

#[derive(Clone, Copy, Debug)]
enum DotError {
    DifferentElementCounts
}

trait Dot<Rhs = Self, Output = Self> {
    fn dot(self , rhs: Rhs) -> Result<Output, DotError>;
}

impl <'a, Iterator1, Element1, Iterator2, Element2, Output> Dot<Iterator2, Output> for Iterator1 
where
    Iterator1: IntoIterator<Item = &'a Element1>,
    &'a Element1: Mul<&'a Element2, Output = Output> + 'a,
    Iterator2: IntoIterator<Item = &'a Element2>,
    &'a Element2: Mul<&'a Element1, Output = Output> + 'a,
    Output: AddAssign + Default
{
    fn dot(self , rhs: Iterator2) -> Result<Output, DotError> {
        let mut output = Output::default();
        let mut self_iter = self.into_iter();
        let mut rhs_iter = rhs.into_iter();
        while let Some(x) = self_iter.next() {
            let Some(rhs_next) = rhs_iter.next() else {
                return Err(DotError::DifferentElementCounts);
            };
            output += x * rhs_next;
        }

        if rhs_iter.next().is_some() {
            return Err(DotError::DifferentElementCounts);
        }

        return Ok(output);
    }
}
