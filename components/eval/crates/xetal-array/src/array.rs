//! The array type: a shape and its items in row-major order.

use crate::{ArrayError, Kind};

#[derive(Debug, Clone)]
pub struct Array<T> {
    pub(crate) shape: Vec<usize>,
    pub(crate) data: Vec<T>,
    /// The kind of the items (T9): what an empty array remembers.
    pub(crate) kind: Kind,
}

impl<T> Array<T> {
    /// An array of `shape` holding `data` in row-major order.
    pub fn new(shape: Vec<usize>, data: Vec<T>) -> Result<Self, ArrayError> {
        if crate::size(&shape)? != data.len() {
            return Err(ArrayError::Length {
                shape,
                len: data.len(),
            });
        }
        Ok(Array {
            shape,
            data,
            kind: Kind::Number,
        })
    }

    /// A vector (rank 1).
    pub fn vector(data: Vec<T>) -> Self {
        Array {
            shape: vec![data.len()],
            data,
            kind: Kind::Number,
        }
    }

    /// A rank-0 array holding one item.
    pub fn scalar(item: T) -> Self {
        Array {
            shape: Vec::new(),
            data: vec![item],
            kind: Kind::Number,
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
