use pyo3::prelude::*;

#[pyfunction]
pub fn integer_sequence(n: usize) -> Vec<usize> {
    let mut res = Vec::new();

    for i in 0..n {
        res.push(i);
    }

    res
}

#[pyfunction]
pub fn fizz_buzz(n: u32) -> String {
    let res: String;
    let is_divisible_by_three = (n % 3) == 0;
    let is_divisible_by_five = (n % 5) == 0;

    if is_divisible_by_three && is_divisible_by_five {
        res = "FizzBuzz".to_string();
    } else if is_divisible_by_three {
        res = "Fizz".to_string();
    } else if is_divisible_by_five {
        res = "Buzz".to_string();
    } else {
        res = n.to_string();
    }

    res
}

#[pymodule]
fn loops_and_conditions(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(integer_sequence, m)?)?;
    m.add_function(wrap_pyfunction!(fizz_buzz, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loops_properly() {
        assert_eq!(integer_sequence(5), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn it_fizz_buzzes_properly() {
        assert_eq!(fizz_buzz(15), "FizzBuzz".to_string());
        assert_eq!(fizz_buzz(4), "4".to_string());
    }
}
