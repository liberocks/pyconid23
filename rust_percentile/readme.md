# Percentile



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
| Name (time in us)         | Min                 | Max                 | Mean               | StdDev            | Median             | IQR               | Outliers | OPS (Kops/s)      | Rounds | Iterations |
|---------------------------|---------------------|---------------------|---------------------|-------------------|---------------------|-------------------|----------|-------------------|--------|------------|
| test_rust_percentile   | 447.7080 (1.0)   | 546.3750 (1.0)   | 465.0953 (1.0)   | 12.0262 (1.11) | 463.4370 (1.0)   | 12.1250 (1.0)   | 558;97   | 2.1501 (1.0)   | 1990   | 1          |
| test_python_percentile | 910.7080 (2.03) | 1,024.5830 (1.88) | 926.5789 (1.99) | 10.8101 (1.0)  | 924.5835 (2.00) | 12.8535 (1.06) | 261;23   | 1.0792 (0.50) | 992    | 1          |
