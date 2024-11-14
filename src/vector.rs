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

/// A generic N sized vector
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector<const N: usize, TYPE: Debug = General> {
    data: [f32; N],
    phantom_type: PhantomData<TYPE>,
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

impl<const N: usize> Default for Vector<N, General> {
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

    pub fn from_data(data: &[f32]) -> Option<Self> {
        if data.len() != N {
            return None;
        }

        let mut vec = Self::zero_vector();

        for col in 0..N {
            vec.data[col] = data[col];
        }

        Some(vec)
    }
}
