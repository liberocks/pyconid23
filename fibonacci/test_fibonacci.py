import fibonacci
import rust_fibonacci


def test_python_fibonacci():
    assert fibonacci.fibonacci(0) == 0
    assert fibonacci.fibonacci(1) == 1
    assert fibonacci.fibonacci(2) == 1
    assert fibonacci.fibonacci(3) == 2
    assert fibonacci.fibonacci(4) == 3
    assert fibonacci.fibonacci(5) == 5
    assert fibonacci.fibonacci(6) == 8
    assert fibonacci.fibonacci(7) == 13
    assert fibonacci.fibonacci(8) == 21
    assert fibonacci.fibonacci(9) == 34
    assert fibonacci.fibonacci(32) == 2178309


def test_rust_fibonacci():
    assert rust_fibonacci.fibonacci(0) == 0
    assert rust_fibonacci.fibonacci(1) == 1
    assert rust_fibonacci.fibonacci(2) == 1
    assert rust_fibonacci.fibonacci(3) == 2
    assert rust_fibonacci.fibonacci(4) == 3
    assert rust_fibonacci.fibonacci(5) == 5
    assert rust_fibonacci.fibonacci(6) == 8
    assert rust_fibonacci.fibonacci(7) == 13
    assert rust_fibonacci.fibonacci(8) == 21
    assert rust_fibonacci.fibonacci(9) == 34
    assert rust_fibonacci.fibonacci(32) == 2178309
