use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "template/template.pest"]
pub struct TemplateParser;

pub struct TemplateConst {
    pub key: String,
    pub value: String,
    pub tag: String,
}

fn parse_set_tag(pair: Pair<Rule>, pair_str: String) -> TemplateConst {
    let mut key = "".to_string();
    let mut expr = "".to_string();

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::ident => key = p.as_str().to_string(),
            Rule::logic_expr => expr = p.as_span().as_str().trim().to_string(),
            _ => {}
        }
    }

    TemplateConst {
        key,
        value: expr,
        tag: pair_str,
    }
}

fn parse_content(pair: Pair<Rule>) -> Vec<TemplateConst> {
    let mut nodes = vec![];

    for p in pair.into_inner() {
        if p.as_rule() == Rule::set_tag {
            let pair_str = p.as_str().to_string();
            nodes.push(parse_set_tag(p, pair_str))
        };
    }

    nodes
}

pub fn parse(input: &str) -> Vec<TemplateConst> {
    let mut nodes = vec![];

    let mut pairs = match TemplateParser::parse(Rule::template, input) {
        Ok(p) => p,
        Err(_e) => {
            return nodes;
        }
    };

    for p in pairs.next().unwrap().into_inner() {
        if p.as_rule() == Rule::content {
            nodes.extend(parse_content(p))
        }
    }

    nodes
}
