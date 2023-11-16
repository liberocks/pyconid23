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
maturin develop
maturin build --release
```

## Run the benchmark and the result (may vary)
```bash
pytest
```

| Name (time in us)                     | Min                  | Max                  | Mean                | StdDev              | Median              | IQR                 | Outliers           | OPS                 | Rounds | Iterations |
|--------------------------------------|----------------------|----------------------|----------------------|----------------------|----------------------|----------------------|----------------------|----------------------|--------|------------|
| test_rust_prime_factorization        | 31.3330 (1.0)        | 57.8330 (1.0)        | 41.0490 (1.0)        | 0.8157 (1.0)        | 40.9160 (1.0)        | 0.1240 (1.0)        | 1455;1591          | 24,361.1358 (1.0)        | 22728  | 1          |
| test_python_prime_factorization      | 3,236.8750 (103.31) | 3,441.4170 (59.51)  | 3,286.2059 (80.06)  | 33.2815 (40.80)  | 3,292.7080 (80.47)  | 57.5830 (464.38)  | 125;1              | 304.3023 (0.01)         | 308    | 1          |
