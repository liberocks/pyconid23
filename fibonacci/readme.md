# Fibonacci



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
| Name (time in ms)                   | Min              | Max              | Mean             | StdDev           | Median           | IQR              | Outliers | OPS              | Rounds | Iterations |
|------------------------------------|-----------------|-----------------|-----------------|-----------------|-----------------|-----------------|----------|------------------|--------|------------|
| test_rust_prime_factorization      | 6.5487 (1.0)     | 6.8048 (1.0)     | 6.5998 (1.0)     | 0.0536 (1.0)     | 6.5873 (1.0)     | 0.0581 (1.0)     | 23;11    | 151.5190 (1.0)    | 149    | 1          |
| test_python_prime_factorization    | 489.9877 (74.82) | 495.8028 (72.86) | 492.7017 (74.65) | 2.2066 (41.20)  | 493.1867 (74.87) | 2.9076 (50.06)  | 2;0      | 2.0296 (0.01)     | 5      | 1          |
