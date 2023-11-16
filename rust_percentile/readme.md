# Percentile

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

| Name (time in us)         | Min                 | Max                 | Mean               | StdDev             | Median             | IQR                | Outliers | OPS (Kops/s)       | Rounds | Iterations |
|--------------------------|---------------------|---------------------|--------------------|---------------------|---------------------|---------------------|----------|---------------------|--------|------------|
| test_rust_percentile     | 449.4160 (1.0)      | 550.2910 (1.0)      | 460.7846 (1.0)     | 10.8895 (1.0)      | 457.2920 (1.0)      | 18.3330 (1.0)       | 303;14   | 2.1702 (1.0)        | 1951   | 1          |
| test_python_percentile   | 914.5410 (2.03)     | 1,080.7500 (1.96)   | 934.6667 (2.03)    | 15.8081 (1.45)     | 932.5000 (2.04)     | 19.2920 (1.05)      | 184;15   | 1.0699 (0.49)       | 967    | 1          |
