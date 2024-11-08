## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

# pipecrafter

**PipeCrafter** is a flexible, chainable data transformation pipeline library for Rust, designed to make data ingestion, transformation, and output easier for Rust projects.

## Features
- **Data Ingestion**: Easily ingest data from various sources (e.g., CSV, JSON).
- **Flexible Transformations**: Chain transformations like map, filter, and custom functions.
- **Output Options**: Output data to files, stdout, or custom sinks.

## Example Usage

```rust
use pipecrafter::{CsvSource, Pipeline, UppercaseTransform, print_output};

fn main() {
    let source = Box::new(CsvSource);
    let pipeline = Pipeline::new(source).add_transformation(Box::new(UppercaseTransform));
    
    let result = pipeline.execute().expect("Pipeline execution failed");
    print_output(result);
}
```
## Getting Started
Add pipecrafter to your Cargo.toml dependencies

Import the necessary components and build your data pipeline!

## License
MIT License

## Author
Ben Santora
