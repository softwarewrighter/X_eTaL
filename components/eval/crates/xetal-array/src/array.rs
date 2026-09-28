//! The array type: a shape and its items in row-major order.

use crate::ArrayError;

#[derive(Debug, Clone, PartialEq)]
pub struct Array<T> {
    shape: Vec<usize>,
    data: Vec<T>,
}

impl<T> Array<T> {
    /// An array of `shape` holding `data` in row-major order.
    pub fn new(shape: Vec<usize>, data: Vec<T>) -> Result<Self, ArrayError> {
        if shape.iter().product::<usize>() != data.len() {
            return Err(ArrayError::Length {
                shape,
                len: data.len(),
            });
        }
        Ok(Array { shape, data })
    }

    /// A vector (rank 1).
    pub fn vector(data: Vec<T>) -> Self {
        Array {
            shape: vec![data.len()],
            data,
        }
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn rank(&self) -> usize {
        self.shape.len()
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }
}

impl<T> Array<T> {
    /// Apply `f` to every item; the shape is kept.
    pub fn map<U, E>(&self, f: impl FnMut(&T) -> Result<U, E>) -> Result<Array<U>, E> {
        let data = self.data.iter().map(f).collect::<Result<_, _>>()?;
        Ok(Array {
            shape: self.shape.clone(),
            data,
        })
    }
}

/// Combine two arrays of the same shape item by item.
pub fn zip<T, U, E: From<ArrayError>>(
    a: &Array<T>,
    b: &Array<T>,
    mut f: impl FnMut(&T, &T) -> Result<U, E>,
) -> Result<Array<U>, E> {
    if a.shape != b.shape {
        return Err(ArrayError::Shape {
            left: a.shape.clone(),
            right: b.shape.clone(),
        }
        .into());
    }
    let data = a
        .data
        .iter()
        .zip(&b.data)
        .map(|(x, y)| f(x, y))
        .collect::<Result<_, _>>()?;
    Ok(Array {
        shape: a.shape.clone(),
        data,
    })
}
