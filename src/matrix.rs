//! Matrix struct definition and operator implementation methods

use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use crate::vector::Vector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct General;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Echelon;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReducedRowEchelon;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stochastic;

/// An M x N matrix
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix<const M: usize, const N: usize, TYPE: Debug = General> {
    data: [[f32; N]; M],
    phantom_type: PhantomData<TYPE>,
}

impl<const M: usize, const N: usize, TYPE: Debug> Default for Matrix<M, N, TYPE> {
    fn default() -> Self {
        Self {
            data: [[0f32; N]; M],
            phantom_type: PhantomData,
        }
    }
}

impl<const M: usize, const N: usize, TYPE: Debug> Index<usize> for Matrix<M, N, TYPE> {
    type Output = [f32];
    fn index(&self, index: usize) -> &Self::Output {
        self.data.index(index)
    }
}

impl<const M: usize, const N: usize, TYPE: Debug> IndexMut<usize> for Matrix<M, N, TYPE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

impl<const M: usize, const N: usize, TYPE: Debug + Copy> std::ops::Mul<f32> for Matrix<M, N, TYPE> {
    type Output = Matrix<M, N, General>;
    fn mul(self, rhs: f32) -> Self::Output {
        self.scalar_multiply(rhs)
    }
}

impl<const M: usize, const N: usize, TYPE: Debug + Copy, OTHER: Debug + Copy>
    std::ops::Add<Matrix<M, N, OTHER>> for Matrix<M, N, TYPE>
{
    type Output = Matrix<M, N, General>;
    fn add(self, rhs: Matrix<M, N, OTHER>) -> Self::Output {
        self.matrix_addition(&rhs)
    }
}

impl<const M: usize, const N: usize, TYPE: Debug + Copy, OTHER: Debug + Copy>
    std::ops::Sub<Matrix<M, N, OTHER>> for Matrix<M, N, TYPE>
{
    type Output = Matrix<M, N, General>;
    fn sub(self, rhs: Matrix<M, N, OTHER>) -> Self::Output {
        self + (rhs * -1f32)
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
}

impl<const M: usize> Matrix<M, M, General> {
    pub fn identity() -> Matrix<M, M, ReducedRowEchelon> {
        let mut mat: Matrix<M, M, ReducedRowEchelon> = Matrix::default();

        for row in 0..M {
            for col in 0..M {
                if row == col {
                    mat[row][col] = 1f32;
                }
            }
        }

        mat
    }
}

impl<const M: usize, const N: usize, TYPE: Debug + Copy> Matrix<M, N, TYPE> {
    pub fn from_rows(vectors: [Vector<N>; M]) -> Matrix<M, N, TYPE> {
        let mut result = Matrix::default();

        for row in 0..M {
            for col in 0..N {
                result[row][col] = vectors[row][col]
            }
        }

        result
    }

    pub fn stochastic_matrix(&self) -> Option<Matrix<M, N, Stochastic>> {
        let columns = self.column_vectors();
        let mut stochastic: Matrix<M, N, Stochastic> = Matrix::default();
        stochastic.data = self.data.clone();

        for vector in columns {
            let _ = vector.probability_vector()?;
        }

        Some(stochastic)
    }

    pub fn column_vectors(&self) -> [Vector<M, crate::vector::General>; N] {
        let mut vectors = [Vector::zero_vector(); N];
        for x in 0..N {
            let vector = &mut vectors[x];
            for y in 0..M {
                vector[y] = self[x][y];
            }
        }
        vectors
    }

    pub fn row_vectors(&self) -> [Vector<N, crate::vector::General>; M] {
        let mut vectors = [Vector::zero_vector(); M];

        for y in 0..M {
            for x in 0..N {
                vectors[y][x] = self[y][x];
            }
        }

        vectors
    }

    pub fn reduced_row_echelon(&self) -> Matrix<M, N, ReducedRowEchelon> {
        let mut rows = self.row_vectors();
        // For each row, get the first non-zero position and scale vector so that is's 1. For all
        // other vectors, if there is a non-zero term in that position, subtract this row from that
        // row scaled by whatever that value is. Then we can do some clean up and move vectors up
        // based on precedence of leading 1s

        for i in 0..M {
            let row = rows[i];
            if let Some(idx) = row.first_non_zero_term() {
                let scalar_reduction = 1f32 / row[idx];
                rows[i] = row * scalar_reduction;

                for k in 0..M {
                    let val = rows[k][idx];
                    if k != i && val != 0f32 {
                        rows[k] = rows[k] - (rows[i] * val)
                    }
                }
            }
        }

        Matrix::from_rows(rows)
    }

    pub fn null_space(&self) -> Vec<Vector<N>> {
        let reduced = self.reduced_row_echelon();
        let mut null_space = Vec::new();

        let mut pivot_columns = vec![None; M];
        let mut free_variables = vec![true; N];

        for (row_idx, row) in reduced.row_vectors().iter().enumerate() {
            if let Some(pivot_idx) = row.first_non_zero_term() {
                pivot_columns[row_idx] = Some(pivot_idx);
                free_variables[pivot_idx] = false;
            }
        }

        for (free_idx, &is_free) in free_variables.iter().enumerate() {
            if is_free {
                let mut null_vector = Vector::zero_vector();

                null_vector[free_idx] = 1.0;

                for (row_idx, pivot_opt) in pivot_columns.iter().enumerate() {
                    if let Some(pivot_idx) = pivot_opt {
                        null_vector[*pivot_idx] = -reduced[row_idx][free_idx];
                    }
                }

                null_space.push(null_vector);
            }
        }

        null_space
    }

    pub fn scalar_multiply(&self, k: f32) -> Matrix<M, N, General> {
        let mut mat = *self;

        for row in 0..M {
            for col in 0..N {
                mat[row][col] *= k;
            }
        }

        let mut result = Matrix::default();
        result.data = mat.data;

        result
    }

    pub fn matrix_addition<LEFT: Debug>(
        &self,
        other: &Matrix<M, N, LEFT>,
    ) -> Matrix<M, N, General> {
        let mut result = Matrix::default();

        for row in 0..M {
            for col in 0..N {
                result[row][col] = self[row][col] + other[row][col];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector;

    use super::Matrix;

    #[test]
    pub fn column_vectors() {
        let matrix = Matrix::from_vectors([
            Vector::from_data([1f32, 2f32]),
            Vector::from_data([1f32, 2f32]),
        ]);

        let vec_space = matrix.column_vectors();
        assert_eq!(
            vec_space,
            [
                Vector::from_data([1f32, 1f32]),
                Vector::from_data([2f32, 2f32])
            ]
        )
    }

    #[test]
    pub fn stochastic_matrix() {
        let matrix =
            Matrix::from_vectors([Vector::from_data([0.5, 0.2]), Vector::from_data([0.5, 0.8])]);

        let non_stochastic_matrix = Matrix::from_vectors([
            Vector::from_data([1f32, 2f32]),
            Vector::from_data([1f32, 2f32]),
        ]);

        assert!(matrix.stochastic_matrix().is_some());
        assert!(non_stochastic_matrix.stochastic_matrix().is_none())
    }

    #[test]
    pub fn scalar_multiplication() {
        let matrix = Matrix::from_vectors([
            Vector::from_data([1f32, 2f32]),
            Vector::from_data([1f32, 2f32]),
        ]);
        let matrix = matrix * 2f32;

        assert_eq!(
            matrix,
            Matrix::from_vectors([
                Vector::from_data([2f32, 4f32]),
                Vector::from_data([2f32, 4f32]),
            ])
        )
    }

    #[test]
    pub fn matrix_addition() {
        let matrix = Matrix::from_vectors([
            Vector::from_data([1f32, 2f32]),
            Vector::from_data([1f32, 2f32]),
        ]);
        let matrix = matrix + matrix;

        assert_eq!(
            matrix,
            Matrix::from_vectors([
                Vector::from_data([2f32, 4f32]),
                Vector::from_data([2f32, 4f32]),
            ])
        )
    }

    #[test]
    pub fn matrix_subtraction() {
        let matrix = Matrix::from_vectors([
            Vector::from_data([1f32, 2f32]),
            Vector::from_data([1f32, 2f32]),
        ]);
        let matrix = matrix - matrix;

        assert_eq!(
            matrix,
            Matrix::from_vectors([
                Vector::from_data([0f32, 0f32]),
                Vector::from_data([0f32, 0f32]),
            ])
        )
    }

    #[test]
    pub fn identity_matrix() {
        let identity: Matrix<2, 2, _> = Matrix::identity();

        assert_eq!(identity.data, [[1f32, 0f32], [0f32, 1f32]])
    }

    #[test]
    pub fn reduce_row_echelon_form() {
        let input = Matrix::from_vectors([
            Vector::from_data([1f32, 2f32]),
            Vector::from_data([5f32, 11f32]),
            Vector::from_data([1f32, 5f32]),
        ]);

        let expected = Matrix::from_vectors([
            Vector::from_data([1f32, 0f32]),
            Vector::from_data([0f32, 1f32]),
            Vector::from_data([-14f32, 3f32]),
        ]);

        let reduced = input.reduced_row_echelon();
        assert_eq!(reduced.data, expected.data)
    }

    #[test]
    pub fn null_space() {
        let input = Matrix::from_vectors([
            Vector::from_data([1f32, 2f32]),
            Vector::from_data([5f32, 11f32]),
            Vector::from_data([1f32, 5f32]),
        ]);

        let null_space = input.null_space();
        assert_eq!(null_space, &[Vector::from_data([14f32, -3f32, 1f32])])
    }

    #[test]
    pub fn identity_matrix_is_reduced_row_echelon() {
        let reduced: Matrix<3, 3, _> = Matrix::identity().reduced_row_echelon();
        let identity: Matrix<3, 3, _> = Matrix::identity();

        assert_eq!(reduced.data, identity.data)
    }
}
