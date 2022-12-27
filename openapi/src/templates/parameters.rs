use std::fmt::Display;

pub struct ParamData {
    pub rust_type: String,
    pub skip_serializing: Option<&'static str>,
}

impl ParamData {
    pub fn new<T: Display>(rust_type: T) -> Self {
        Self { rust_type: rust_type.to_string(), skip_serializing: None }
    }

    pub fn skip_serializing(mut self, skip_serializing: &'static str) -> Self {
        self.skip_serializing = Some(skip_serializing);
        self
    }
}
