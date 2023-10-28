def factorize(n: int) -> list[int]:
    """
    Returns the prime factorization of a given integer.

    Args:
      n (int): The integer to be factorized.

    Returns:
      list[int]: A list of prime factors of the given integer.
    """
    factors: list[int] = []
    i: int = 2

    while i * i <= n:
        if n % i:
            i += 1
        else:
            n //= i
            factors.append(i)

    if n > 1:
        factors.append(n)

    return factors
