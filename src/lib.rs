// Import the necessary modules
pub mod data_source;
pub mod transformations;
pub mod pipeline;
pub mod output;

pub use data_source::*;
pub use transformations::*;
pub use pipeline::*;
pub use output::*;

// Basic test to ensure the library builds correctly
#[cfg(test)]
mod tests {
    #[test]
    fn basic_test() {
        assert_eq!(2 + 2, 4);  // Simple test to confirm tests run
    }
}
