def fibonacci(n: int) -> int:
    """
    Returns the nth number in the Fibonacci sequence.

    Parameters:
    n (int): The index of the number to be returned in the Fibonacci sequence.

    Returns:
    int: The nth number in the Fibonacci sequence.

    """
    if n == 0:
        return 0
    elif n == 1:
        return 1
    else:
        return fibonacci(n - 1) + fibonacci(n - 2)
