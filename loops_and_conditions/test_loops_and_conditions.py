from loops_and_conditions import integer_sequence, fizz_buzz

def test_integer_sequence():
    assert integer_sequence(10) == [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]


def test_fizz_buzz():
    assert fizz_buzz(3) == "Fizz"
    assert fizz_buzz(5) == "Buzz"
    assert fizz_buzz(4) == "4"
    assert fizz_buzz(15) == "FizzBuzz"
