use jsonschema::JSONSchema;
use pyo3::prelude::*;
use serde_json::{Result, Value};

#[pyfunction]
pub fn validate_json(schema_str: &str, data_str: &str) -> bool {
    let maybe_schema = serde_json::from_str(schema_str);
    let maybe_data = serde_json::from_str(data_str);

    if maybe_schema.is_err() || maybe_data.is_err() {
        return false;
    }

    let schema: Value = maybe_schema.unwrap();
    let data: Value = maybe_data.unwrap();

    let compiled = JSONSchema::compile(&schema).unwrap();
    let result = compiled.is_valid(&data);

    result
}

#[pymodule]
fn use_rust_library(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate_json, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let schema_str = r#"
        {
            "type" : "object",
            "properties" : {
                "price" : {
                    "type" : "number"
                },
                "name" : {
                    "type" : "string"
                }
            }
        }
        "#;

        let data_str = r#"
        {
            "name": "Eggs",
            "price": 34.99
        }
        "#;

        let result = validate_json(schema_str, data_str);

        assert_eq!(result, true);
    }
}
