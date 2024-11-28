//! Vector implementation
use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Probability;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct General;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Unit;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProbabilityRegular;

/// A generic N sized vector
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector<const N: usize, TYPE: Debug = General> {
    data: [f32; N],
    phantom_type: PhantomData<TYPE>,
}

impl<const N: usize, TYPE: Debug> std::ops::Mul<f32> for Vector<N, TYPE> {
    type Output = Vector<N, General>;
    fn mul(self, rhs: f32) -> Self::Output {
        self.scalar_multiply(rhs)
    }
}

impl<const N: usize, TYPE: Debug, OTHER: Debug> std::ops::Sub<Vector<N, OTHER>>
    for Vector<N, TYPE>
{
    type Output = Vector<N, General>;
    fn sub(self, rhs: Vector<N, OTHER>) -> Self::Output {
        self + (rhs * -1f32)
    }
}

impl<const N: usize, TYPE: Debug, OTHER: Debug> std::ops::Add<Vector<N, OTHER>>
    for Vector<N, TYPE>
{
    type Output = Vector<N, General>;
    fn add(self, rhs: Vector<N, OTHER>) -> Self::Output {
        self.vector_addition(&rhs)
    }
}

impl<const N: usize, TYPE: Debug, Idx: SliceIndex<[f32], Output = f32>> Index<Idx>
    for Vector<N, TYPE>
{
    type Output = f32;
    fn index(&self, index: Idx) -> &Self::Output {
        self.data.index(index)
    }
}

impl<const N: usize, TYPE: Debug, Idx: SliceIndex<[f32], Output = f32>> IndexMut<Idx>
    for Vector<N, TYPE>
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

impl<const N: usize, TYPE: Debug> Default for Vector<N, TYPE> {
    fn default() -> Self {
        Self {
            data: [0f32; N],
            phantom_type: PhantomData,
        }
    }
}

impl<const N: usize> Vector<N, General> {
    pub fn zero_vector() -> Self {
        Self::default()
    }

    pub fn from_data(data: [f32; N]) -> Self {
        let mut vec = Self::zero_vector();

        for col in 0..N {
            vec.data[col] = data[col];
        }

        vec
    }
}

impl<const N: usize, TYPE: Debug> Vector<N, TYPE> {
    pub fn magnitude(&self) -> f32 {
        self.data.iter().map(|val| val.powi(2)).sum::<f32>().sqrt()
    }

    pub fn unit_vector(&self) -> Vector<N, Unit> {
        let mut unit_vec = Vector::default();
        let magnitude = self.magnitude();

        for i in 0..N {
            unit_vec[i] = self[i] / magnitude;
        }

        unit_vec
    }

    pub fn probability_vector(&self) -> Option<Vector<N, Probability>> {
        if self.data.iter().sum::<f32>() == 1.0 {
            let mut new_vec = Vector::default();
            new_vec.data = self.data;
            Some(new_vec)
        } else {
            None
        }
    }

    pub fn contains_zero(&self) -> bool {
        self.data.contains(&0f32)
    }

    pub fn vector_addition<OTHER: Debug>(&self, other: &Vector<N, OTHER>) -> Vector<N, General> {
        let mut result = Vector::default();

        for i in 0..N {
            result[i] = self[i] + other[i];
        }

        result
    }

    pub fn scalar_multiply(&self, k: f32) -> Vector<N, General> {
        let mut result = Vector::default();

        for i in 0..N {
            result[i] = self[i] * k;
        }

        result
    }
}

impl<const N: usize> Vector<N, Probability> {
    pub fn regular(&self) -> Option<Vector<N, ProbabilityRegular>> {
        if self.data.iter().filter(|element| **element > 0.0).count() == self.data.len() {
            let mut new_vec = Vector::default();
            new_vec.data = self.data;

            Some(new_vec)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vector;

    #[test]
    fn regular_probability_vector() {
        let vector = Vector::from_data([0.1, 0.1, 0.8]);
        let p_vector = vector.probability_vector().expect("Probability vector");
        assert!(p_vector.regular().is_some())
    }

    #[test]
    fn nonregular_probability_vector() {
        let vector = Vector::from_data([0.2, 0.0, 0.8]);
        let p_vector = vector.probability_vector().expect("Probability vector");
        assert!(p_vector.regular().is_none())
    }

    #[test]
    fn scalar_multiplication() {
        let vector = Vector::from_data([1f32, 2f32, 3f32]);
        let vector = vector * 3f32;

        assert_eq!(vector, Vector::from_data([3f32, 6f32, 9f32]))
    }

    #[test]
    fn vector_addition() {
        let vector = Vector::from_data([1f32, 2f32, 3f32]);
        let vector = vector + vector;

        assert_eq!(vector, Vector::from_data([2f32, 4f32, 6f32]))
    }

    #[test]
    fn vector_subtraction() {
        let vector = Vector::from_data([1f32, 2f32, 3f32]);
        let vector = vector - vector;

        assert_eq!(vector, Vector::from_data([0f32, 0f32, 0f32]))
    }
}
