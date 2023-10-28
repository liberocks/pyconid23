class Percentile(object):
    def __init__(self, numbers: list[int]):
        self.numbers = numbers

    def percentile(self, n: int) -> float:
        if not 0 <= n <= 100:
            raise ValueError("Percentile must be between 0 and 100")
        else:
            sorted_numbers = sorted(self.numbers)
            index = (n / 100) * (len(sorted_numbers) - 1)
            if index.is_integer():
                return sorted_numbers[int(index)]
            else:
                lower = sorted_numbers[int(index)]
                upper = sorted_numbers[int(index) + 1]
                return (lower + upper) / 2
