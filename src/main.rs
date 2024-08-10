#[macro_use]
extern crate clap;

extern crate chrono;
extern crate fake;
extern crate http;
extern crate semver;
extern crate serde;

use std::env;
use std::fs;
use std::path::PathBuf;

use askama_parser::{Ast, Syntax};

mod cli;
#[macro_use]
mod macros;
mod domain;
mod template;

use crate::template::{
    generator::Generator,
    generator::AstLevel,
};

fn lang_env() -> Option<String> {
    match std::env::var("LANG") {
        Ok(_l) => _l.find('.').map(|pos| _l[0..pos].to_lowercase()),
        Err(_) => None,
    }
}

fn main() {
    let matches = cli::build_cli(crate_name!(), crate_version!()).get_matches();

    if let Some(template_m) = matches.subcommand_matches("template") {
        if let Some(input) = template_m.get_one::<PathBuf>("input") {
            let contents =
                fs::read_to_string(input).expect("Should have been able to read the file");

            let ast = Ast::from_str(&contents, &Syntax::default()).unwrap();
            
            let mut g = Generator::new(template_m);
            
            if let Err(err) = g.handle(ast.nodes(), AstLevel::Top) {
                eprint!("{}", err);
                return;
            }
            println!("{}", g.render());
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
