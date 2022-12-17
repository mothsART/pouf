use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "template/template.pest"]
pub struct TemplateParser;
