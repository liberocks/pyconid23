import prime_factorization
import rust_prime_factorization


def test_python_prime_factorization(benchmark):
    result = benchmark(prime_factorization.factorize, 2147483647)

    assert result == [2147483647]


def test_rust_prime_factorization(benchmark):
    result = benchmark(rust_prime_factorization.factorize, 2147483647)

    assert result == [2147483647]
