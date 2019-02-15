use std::ops::{Index,IndexMut};

macro_rules! matrix {
    ($w: expr, $h: expr) => {
        Matrix::new([[0.0; $w]; $h])
    }
}

/// A dynamically-sized matrix.
/// ```nahar
/// # A dynamically-sized matrix
/// Matrix
///     [Number] entries
///     
/// # Create a matrix
/// [Float32] vector: 1 1
/// [Float32] matrix: 1 0
///                   0 1
/// [Float32] matrix: 1 0, 0 1
/// [Float32] cube:   1 0, 0 0
///                   0 0, 0 1
/// ```
pub struct Matrix<T, U>
    where T: Index<U> + IndexMut<U>, U: Index<f32> + IndexMut<f32>
{
    entries: Vec<T>,
    dimensions: Vec<u8>,

    entries: Vec<Vec<T>>,
    __: std::marker::PhantomData<U>,
}

impl<T, U> Matrix<T, U>
    where T: Index<U> + IndexMut<U>, U: Index<f32> + IndexMut<f32>
{
    /// Returns true if it's a square matrix.
    pub fn square(&self) -> bool {
        self
    }

    /// Create a new matrix.
    pub fn identity(w: usize, h: usize) -> Matrix<T, U> {
        
    }

    /// Create a new matrix from a slice.
    pub fn new(array: T) -> Matrix<T, U> {
        Matrix {
            entries: array,
            __: std::marker::PhantomData,
        }
    }
}
