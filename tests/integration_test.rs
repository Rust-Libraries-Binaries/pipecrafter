use pipecrafter::{CsvSource, Pipeline, UppercaseTransform, print_output};

#[test]
fn pipeline_integration_test() {
    let source = Box::new(CsvSource);
    let pipeline = Pipeline::new(source).add_transformation(Box::new(UppercaseTransform));
    
    let result = pipeline.execute().expect("Pipeline execution failed");
    
    // For now, just check if the result is non-empty as a basic test
    assert!(!result.is_empty());
    print_output(result);
}
