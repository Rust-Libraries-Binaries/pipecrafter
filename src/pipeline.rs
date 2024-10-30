use crate::{DataSource, Transformation};

pub struct Pipeline {
    source: Box<dyn DataSource>,
    transformations: Vec<Box<dyn Transformation>>,
}

impl Pipeline {
    pub fn new(source: Box<dyn DataSource>) -> Self {
        Pipeline { source, transformations: vec![] }
    }

    pub fn add_transformation(mut self, transform: Box<dyn Transformation>) -> Self {
        self.transformations.push(transform);
        self
    }

    pub fn execute(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let data = self.source.read()?;
        Ok(self.transformations.iter().fold(data, |data, transform| {
            transform.apply(data)
        }))
    }
}
