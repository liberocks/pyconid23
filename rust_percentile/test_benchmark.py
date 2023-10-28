import percentile
import rust_percentile

from helper import known_random_list, known_75th_percentile


def test_python_percentile(benchmark):
    obj = percentile.Percentile(known_random_list)
    result = benchmark(obj.percentile, 75)

    assert result == known_75th_percentile


def test_rust_percentile(benchmark):
    obj = rust_percentile.Percentile(known_random_list)
    result = benchmark(obj.percentile, 75)

    assert result == known_75th_percentile
