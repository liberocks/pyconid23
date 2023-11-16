use pyo3::prelude::*;

#[pyfunction]
pub fn sort_array(arr: Vec<i32>) -> Vec<i32> {
    let mut res = arr.clone();
    res.sort();

    res
}

#[pyfunction]
pub fn reverse_array(arr: Vec<i32>) -> Vec<i32> {
    let mut res = arr.clone();
    res.reverse();

    res
}

#[pyfunction]
pub fn append_array(arr: Vec<i32>, item: i32) -> Vec<i32> {
    let mut res = arr.clone();
    res.push(item);

    res
}

#[pyfunction]
pub fn insert_array(arr: Vec<i32>, index: usize, item: i32) -> Vec<i32> {
    let mut res = arr.clone();
    res.insert(index, item);

    res
}

#[pymodule]
fn array_operation(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sort_array, m)?)?;
    m.add_function(wrap_pyfunction!(reverse_array, m)?)?;
    m.add_function(wrap_pyfunction!(append_array, m)?)?;
    m.add_function(wrap_pyfunction!(insert_array, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_properly_do_sorting() {
        let arr = vec![2, 5, 1, 4, 3];
        let expected = vec![1, 2, 3, 4, 5];

        let result = sort_array(arr);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_properly_do_reversal() {
        let arr = vec![9, 8, 7, 6];
        let expected = vec![6, 7, 8, 9];

        let result = reverse_array(arr);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_properly_do_append() {
        let arr = vec![9, 8, 7, 6];
        let item = 5;
        let expected = vec![9, 8, 7, 6, 5];

        let result = append_array(arr, item);

        assert_eq!(result, expected);
    }

    #[test]
    fn it_properly_do_insert() {
        let arr = vec![9, 7, 6];
        let item = 8;
        let index = 1;
        let expected = vec![9, 8, 7, 6];

        let result = insert_array(arr, index, item);

        assert_eq!(result, expected);
    }
}
