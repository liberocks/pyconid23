use pyo3::prelude::*;

#[pyfunction]
pub fn integer_addition(left: i32, right: i32) -> i32 {
    left + right
}

#[pyfunction]
pub fn float_subtraction(left: f32, right: f32) -> f32 {
    left - right
}

#[pyfunction]
pub fn mixed_multiplication(left: i32, right: f32) -> f32 {
    left as f32 * right
}

#[pymodule]
fn type_conversion(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(integer_addition, m)?)?;
    m.add_function(wrap_pyfunction!(float_subtraction, m)?)?;
    m.add_function(wrap_pyfunction!(mixed_multiplication, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_properly_do_addition() {
        let left = 1;
        let right = 2;
        let expected = 3;

        let result = integer_addition(left, right);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_properly_do_subtraction() {
        let left = 1.0;
        let right = 2.0;
        let expected = -1.0;

        let result = float_subtraction(left, right);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_properly_do_multiplication() {
        let left = 1;
        let right = 2.0;
        let expected = 2.0;

        let result = mixed_multiplication(left, right);

        assert_eq!(result, expected);
    }
}
