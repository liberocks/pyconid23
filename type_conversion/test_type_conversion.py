from type_conversion import integer_addition, float_subtraction, mixed_multiplication


def test_integer_addition():
    left: int = 1
    right: int = 2
    expected: int = 3

    result = integer_addition(left, right)
    result_type = type(result)

    assert result == expected
    assert result_type == int


def test_float_subtraction():
    left: float = 1
    right: float = 2
    expected: float = -1

    result = float_subtraction(left, right)
    result_type = type(result)

    assert result == expected
    assert result_type == float


def test_mixed_multiplication():
    left: int = 1
    right: float = 2
    expected: float = 2

    result = mixed_multiplication(left, right)
    result_type = type(result)

    assert result == expected
    assert result_type == float
