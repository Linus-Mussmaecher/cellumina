pub enum CelluminaError {
    IndexOutOfBoundsError(u32, u32, u32, u32),
}

impl std::fmt::Display for CelluminaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CelluminaError::IndexOutOfBoundsError(row, col, max_row, max_col) => {
                write!(
                    f,
                    "Index ({row}, {col}) out of bounds for grid of size ({max_row}, {max_col})."
                )?;
            }
        }
        Ok(())
    }
}
