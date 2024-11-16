use pyo3::prelude::*;

mod datetime;
mod integration;

/// A Python module implemented in Rust.
#[pymodule]
mod pyo3_test {
    #[pymodule_export]
    use crate::datetime::DateTime;
}
