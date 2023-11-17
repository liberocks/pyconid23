import fibonacci
import rust_fibonacci


def test_python_fibonacci(benchmark):
    result = benchmark(fibonacci.fibonacci, 32)

    assert result == 2178309


def test_rust_fibonacci(benchmark):
    result = benchmark(rust_fibonacci.fibonacci, 32)

    assert result == 2178309
