# Prime Factorization

Prime factorization is finding which prime numbers multiply together to make the original number. The complexity of this algorithm is O(sqrt(n)) because the loop runs from 2 to sqrt(n). In python, this algorithm can be implemented as follows:

```python
def factorize(n: int) -> list[int]:
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
```

While in Rust, it can be implemented as follows:

```rust
pub fn factorize(n: u32) -> Vec<u32> {
    let mut factors = Vec::<u32>::new();
    let mut i = 2;
    let mut num = n;

    while i * i <= n {
        if num % i != 0 {
            i += 1;
        } else {
            num /= i;
            factors.push(i);
        }
    }
    if num > 1 {
        factors.push(num);
    }

    factors
}
```

## Installation
```bash
poetry shell
poetry install
```

## Build the Rust implementation
```bash
maturin build --release
```

## Run the benchmark and the result (may vary)
```bash
pytest
```

| Name (time in ns)                     | Min                 | Max                 | Mean               | StdDev             | Median             | IQR                | Outliers     | OPS (Kops/s)       | Rounds  | Iterations |
|--------------------------------------|---------------------|---------------------|--------------------|---------------------|---------------------|---------------------|--------------|---------------------|---------|------------|
| test_rust_prime_factorization        | 841.6500 (1.0)      | 1,685.4000 (1.0)    | 852.2027 (1.0)     | 21.5459 (1.0)       | 850.0000 (1.0)      | 4.1000 (1.0)        | 1015;4391   | 1,173.4298 (1.0)    | 57691   | 20         |
| test_python_prime_factorization      | 1,116.5999 (1.33)   | 19,325.0000 (11.47) | 1,169.7934 (1.37)  | 73.1295 (3.39)      | 1,166.6001 (1.37)   | 16.6001 (4.05)     | 813;7292    | 854.8518 (0.73)    | 171409  | 5          |
