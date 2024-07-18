use anyhow::{anyhow, Result};
use std::ops::{Add, AddAssign, Deref, Mul};

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }

    // pub fn len(&self) -> usize {
    //     self.data.len()
    // }

    // pub fn iter(&self) -> std::slice::Iter<T> {
    //     self.data.iter()
    // }
}

/**
 * 实现解引用，自动转化为Vec<T>
 */
impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// impl<T> std::ops::Index<usize> for Vector<T>{
//     type Output = T;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.data[index]
//     }
// }

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.len() != b.len() {
        //a.len() => a.data.len()(Deref trait)
        return Err(anyhow!("dot product error: a.len() != b.len()"));
    }
    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}
