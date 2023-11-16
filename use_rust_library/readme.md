# Use Rust library in Python



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

## Run the test
```bash
pytest
```