use pyo3::prelude::*;
use crate::datetime::DateTimeError;

impl std::convert::From<DateTimeError> for PyErr {
    fn from(err: DateTimeError) -> PyErr {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Datetime error: {}", err.message))
    }
}
