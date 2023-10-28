import percentile
import rust_percentile


def test_python_percentile():
    obj = percentile.Percentile([5, 3, 1, 2, 4])

    assert obj.percentile(0) == 1
    assert obj.percentile(25) == 2
    assert obj.percentile(50) == 3
    assert obj.percentile(75) == 4
    assert obj.percentile(100) == 5


def test_rust_percentile():
    obj = rust_percentile.Percentile([5, 3, 1, 2, 4])

    assert obj.percentile(0) == 1
    assert obj.percentile(25) == 2
    assert obj.percentile(50) == 3
    assert obj.percentile(75) == 4
    assert obj.percentile(100) == 5
