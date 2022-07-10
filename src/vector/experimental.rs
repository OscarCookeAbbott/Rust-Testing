use std::cmp::*;
use std::fmt::*;
use std::ops::*;

pub struct Vector<T>(pub Vec<T>);

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Add for Vector<T>
where
    T: Copy + Add<Output = T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result: Self = Vector{0: vec![]};
        for i in 0..self.len() {
            result.push(self[i] + rhs[i]);
        }
        result
    }
}
