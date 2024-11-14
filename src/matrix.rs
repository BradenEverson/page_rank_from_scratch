//! Matrix struct definition and operator implementation methods

use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use crate::vector::Vector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct General;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Echelon;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReducedRowEchelon;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stochastic<STATE>(PhantomData<STATE>);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Regular;

/// An M x N matrix
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<const M: usize, const N: usize, TYPE: Debug = General> {
    data: [[f32; N]; M],
    phantom_type: PhantomData<TYPE>,
}

impl<const M: usize, const N: usize> Default for Matrix<M, N, General> {
    fn default() -> Self {
        Self {
            data: [[0f32; N]; M],
            phantom_type: PhantomData,
        }
    }
}

impl<const M: usize, const N: usize> Matrix<M, N, General> {
    pub fn from_vectors(vecs: [Vector<M>; N]) -> Self {
        let mut mat = Self::zero_matrix();

        for row in 0..M {
            for col in 0..N {
                mat[row][col] = vecs[col][row]
            }
        }

        mat
    }

    pub fn zero_matrix() -> Self {
        Self::default()
    }

    pub fn from_data(data: &[f32]) -> Option<Self> {
        if data.len() != M * N {
            return None;
        }

        let mut mat = Self::zero_matrix();

        let mut idx = 0;

        for row in 0..M {
            for col in 0..N {
                mat[row][col] = data[idx];
                idx += 1;
            }
        }

        Some(mat)
    }

    pub fn scalar_multiply(&self, k: f32) -> Self {
        let mut mat = *self;

        for row in 0..M {
            for col in 0..N {
                mat[row][col] *= k;
            }
        }

        mat
    }
}

impl<const M: usize, const N: usize> Index<usize> for Matrix<M, N, General> {
    type Output = [f32];
    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<const M: usize, const N: usize> IndexMut<usize> for Matrix<M, N, General> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}
