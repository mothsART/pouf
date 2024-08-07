use std::any::Any;

pub trait TemplateObject {
    fn get_property(&self, key: &str) -> Option<&dyn Any>;
}
