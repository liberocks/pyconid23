from use_rust_library import validate_json
from json import dumps


def test_use_rust_library():
    schema = dumps(
        {
            "type": "object",
            "properties": {
                "price": {
                    "type": "number",
                },
                "name": {
                    "type": "string",
                },
            },
        }
    )
    value = dumps(
        {
            "price": 34.99,
            "name": "Eggs",
        }
    )
    assert validate_json(schema, value) == True
