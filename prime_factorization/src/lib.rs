use pyo3::prelude::*;

#[pyfunction]
pub fn factorize(n: u32) -> Vec<u32> {
    let mut factors = Vec::<u32>::new();
    let mut i = 2;
    let mut num = n;

    while i * i <= n {
        if num % i != 0 {
            i += 1;
        } else {
            num /= i;
            factors.push(i);
        }
    }
    if num > 1 {
        factors.push(num);
    }

    factors
}

#[pymodule]
fn rust_prime_factorization(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(factorize, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(prime_factorization(2), vec![2]);
        assert_eq!(prime_factorization(10), vec![2, 5]);
        assert_eq!(prime_factorization(29), vec![29]);
        assert_eq!(
            prime_factorization(110000),
            vec![2, 2, 2, 2, 5, 5, 5, 5, 11]
        );
    }
}
