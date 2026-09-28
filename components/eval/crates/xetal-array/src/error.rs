//! Array errors; they convert into `xetal_base::Diagnostic`.

use xetal_base::Diagnostic;

/// Pipeline stage name used in diagnostics.
pub const STAGE: &str = "array";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayError {
    /// The data does not fill the shape.
    Length { shape: Vec<usize>, len: usize },
    /// Two arrays combined element by element have different shapes.
    Shape { left: Vec<usize>, right: Vec<usize> },
}

fn dims(shape: &[usize]) -> String {
    let parts: Vec<String> = shape.iter().map(ToString::to_string).collect();
    parts.join(" ")
}

impl From<ArrayError> for Diagnostic {
    fn from(err: ArrayError) -> Self {
        match err {
            ArrayError::Length { shape, len } => Diagnostic::new(
                "length-mismatch",
                format!("{len} items do not fill shape {}", dims(&shape)),
            ),
            ArrayError::Shape { left, right } => Diagnostic::new(
                "shape-mismatch",
                format!("shapes differ: {} and {}", dims(&left), dims(&right)),
            ),
        }
    }
}
