use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
pub fn division(left: i32, right: i32) -> PyResult<i32> {
    match left.checked_div(right) {
        Some(result) => Ok(result),
        None => Err(PyValueError::new_err(
            "Division by zero or overflow occurred",
        )),
    }
}

#[pymodule]
fn error_in_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(division, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_raises_an_error() {
        assert!(division(1, 0).is_err());
    }

    #[test]
    fn it_doesnt_raise_an_error() {
        assert!(division(1, 1).is_ok());
    }
}
