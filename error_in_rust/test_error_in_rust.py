import pytest

from error_in_rust import division

def test_positive_case():
    assert division(10, 1) == 10

def test_negative_case():
    with pytest.raises(Exception):
        division(10, 0)
