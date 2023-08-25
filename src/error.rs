/// Internal Cellumina library error type.
#[derive(Debug)]
pub enum CelluminaError {
    /// An Index-Out-Of-Bounds-Error when accessing the underlying state grid of an automaton.
    CustomError(String),
    IndexOutOfBoundsError(u32, u32, u32, u32),
    IOError(std::io::Error),
    ImageError(image::error::ImageError),
}

impl std::fmt::Display for CelluminaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CelluminaError::IndexOutOfBoundsError(row, col, max_row, max_col) => {
                write!(
                    f,
                    "Index ({row}, {col}) out of bounds for grid of size ({max_row}, {max_col})."
                )?;
                Ok(())
            }
            Self::CustomError(msg) => {
                write!(f, "{msg}")?;
                Ok(())
            }
            CelluminaError::IOError(e) => e.fmt(f),
            CelluminaError::ImageError(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for CelluminaError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<image::error::ImageError> for CelluminaError {
    fn from(value: image::error::ImageError) -> Self {
        Self::ImageError(value)
    }
}
