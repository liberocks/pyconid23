import prime_factorization
import rust_prime_factorization


def test_python_prime_factorization():
    assert prime_factorization.factorize(2) == [2]
    assert prime_factorization.factorize(10) == [2, 5]
    assert prime_factorization.factorize(29) == [29]
    assert prime_factorization.factorize(110000) == [2, 2, 2, 2, 5, 5, 5, 5, 11]
    assert prime_factorization.factorize(2147483647) == [2147483647]


def test_rust_prime_factorization():
    assert rust_prime_factorization.factorize(2) == [2]
    assert rust_prime_factorization.factorize(10) == [2, 5]
    assert rust_prime_factorization.factorize(29) == [29]
    assert rust_prime_factorization.factorize(110000) == [2, 2, 2, 2, 5, 5, 5, 5, 11]
    assert rust_prime_factorization.factorize(2147483647) == [2147483647]
