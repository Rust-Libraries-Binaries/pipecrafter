pub trait Transformation {
    fn apply(&self, input: Vec<String>) -> Vec<String>;
}

pub struct UppercaseTransform;

impl Transformation for UppercaseTransform {
    fn apply(&self, input: Vec<String>) -> Vec<String> {
        input.into_iter().map(|s| s.to_uppercase()).collect()
    }
}
