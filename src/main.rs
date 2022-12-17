#[macro_use]
extern crate clap;

extern crate chrono;
extern crate fake;
extern crate http;
extern crate semver;
extern crate tera;
extern crate serde;

use std::env;
use std::fs;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use pest::Parser;
use pest::iterators::Pair;
use tera::{Context, Tera};

mod cli;
#[macro_use]
mod macros;
mod domain;
mod template;

use crate::template::template::{Rule, TemplateParser};

fn lang_env() -> Option<String> {
    match std::env::var("LANG") {
        Ok(_l) => _l.find('.').map(|pos| _l[0..pos].to_lowercase()),
        Err(_) => None,
    }
}

pub struct TemplateConst {
   key: String,
   value: i32,
   tag: String
}

fn parse_set_tag(pair: Pair<Rule>, pair_str: String) -> TemplateConst {
    let mut key = "".to_string();
    let mut expr = 0;

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::ident => key = p.as_str().to_string(),
            Rule::logic_expr => expr = p.as_span().as_str().trim().parse::<i32>().unwrap_or(0),
            _ => {},
        }
    }

    TemplateConst {
        key: key,
        value: expr,
        tag : pair_str
    }
}

fn parse_content(pair: Pair<Rule>) -> Vec<TemplateConst> {
    let mut nodes = vec![];

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::set_tag => {
                let pair_str = p.as_str().to_string();
                nodes.push(parse_set_tag(p, pair_str))
            },
            _ => {}
        };
    }

    nodes
}

pub fn parse(input: &str) -> Vec<TemplateConst> {
    let mut nodes = vec![];

    let mut pairs = match TemplateParser::parse(Rule::template, input) {
        Ok(p) => {
            p
        },
        Err(_e) => {
            return nodes;
        }
    };

    for p in pairs.next().unwrap().into_inner() {
        match p.as_rule() {
            Rule::content => nodes.extend(parse_content(p)),
            _ => {},
        }
    }

    nodes
}

use clap::ArgMatches;
use crate::fake::Fake;
use fake::faker::name::raw::{FirstName, LastName};

#[derive(Serialize, Deserialize, Debug)]
struct People {
    first_name: String,
    last_name: String,
}

impl People {
    fn create(arg: &ArgMatches) -> People {
        People {
            first_name: format!("{:?}", each!(FirstName, arg)),
            last_name: format!("{:?}", each!(LastName, arg)),
        }
    }
}


fn main() {
    let matches = cli::build_cli(crate_name!(), crate_version!()).get_matches();
    if let Some(template_m) = matches.subcommand_matches("template") {
        if let Some(input) = template_m.get_one::<PathBuf>("input") {
            let mut contents = fs::read_to_string(input).expect("Should have been able to read the file");

            let nodes = parse(&contents);

            let mut nb_peoples = 0;
            for n in nodes {
                match n.key.as_str() {
                    "peoples_nb" => {
                        nb_peoples = n.value;
                        contents = contents.replace(&n.tag, "");
                    },
                    _ => {}
                }
            }

            let mut peoples = vec![];
            if nb_peoples > 0 {
                for _ in 0..nb_peoples {
                    peoples.push(People::create(template_m));
                }
            }

            let mut context = Context::new();
            context.insert("peoples", &mut peoples);

            let result = Tera::one_off(&contents, &context, true);
            println!("{:?}", result);
            //let mut tera = Tera::default();
            //tera.add_template_file(contents, Some("essai"));
        }
        return;
    }

    domain::address::run(&matches);
    domain::administrative::run(&matches);
    domain::auto::run(&matches);
    domain::barcode::run(&matches);
    domain::color::run(&matches);
    domain::filesystem::run(&matches);
    domain::finance::run(&matches);
    domain::http::run(&matches);
    domain::internet::run(&matches);
    domain::lorem::run(&matches);
    domain::people::run(&matches);
    domain::time::run(&matches);
}
