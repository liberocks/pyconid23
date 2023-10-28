use pyo3::prelude::*;

#[pyclass]
struct Percentile {
    numbers: Vec<i32>,
}

#[pymethods]
impl Percentile {
    #[new]
    fn new(numbers: Vec<i32>) -> Percentile {
        Percentile { numbers }
    }

    fn percentile(&self, n: i32) -> f32 {
        if !(0..=100).contains(&n) {
            panic!("Percentile must be between 0 and 100");
        } else {
            let mut sorted_numbers = self.numbers.clone();
            sorted_numbers.sort();
            let index = (n as f32 / 100.0) * (sorted_numbers.len() as f32 - 1.0);
            if index.fract() == 0.0 {
                sorted_numbers[index as usize] as f32
            } else {
                let lower = sorted_numbers[index as usize];
                let upper = sorted_numbers[(index as usize) + 1];
                (lower as f32 + upper as f32) / 2.0
            }
        }
    }
}

#[pymodule]
fn rust_percentile(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Percentile>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let percentile = Percentile::new(vec![5, 3, 1, 2, 4]);
        
        assert_eq!(percentile.percentile(0), 1.0);
        assert_eq!(percentile.percentile(25), 2.0);
        assert_eq!(percentile.percentile(50), 3.0);
        assert_eq!(percentile.percentile(75), 4.0);
        assert_eq!(percentile.percentile(100), 5.0);
    }
}
