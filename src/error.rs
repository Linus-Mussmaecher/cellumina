use thiserror::Error;

/// Internal Cellumina library error type.
#[derive(Debug, Error)]
pub enum CelluminaError {
    /// Any different type of error.
    #[error("{0}")]
    CustomError(String),
    /// An Index-Out-Of-Bounds-Error when accessing the underlying state grid of an automaton.
    #[error("index ({0}, {1}) out of bounds for state grid of size ({2}, {3})")]
    IndexOutOfBoundsError(u32, u32, u32, u32),
    /// Error passed on from std::io.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    /// Error passed on from the image crate.
    #[error(transparent)]
    ImageError(#[from] image::error::ImageError),
}
