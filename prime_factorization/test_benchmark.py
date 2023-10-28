import prime_factorization
import rust_prime_factorization


def test_python_prime_factorization(benchmark):
    result = benchmark(prime_factorization.factorize, 999999)

    assert result == [3, 3, 3, 7, 11, 13, 37]


def test_rust_prime_factorization(benchmark):
    result = benchmark(rust_prime_factorization.factorize, 999999)

    assert result == [3, 3, 3, 7, 11, 13, 37]
