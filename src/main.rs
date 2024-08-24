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

use crate::template::{generator::AstLevel, generator::Generator};
use pouf::lang_env;

fn main() {
    let matches = cli::build_cli(crate_name!(), crate_version!()).get_matches();

    if let Some(template_m) = matches.subcommand_matches("template") {
        if let Some(input) = template_m.get_one::<PathBuf>("input") {
            if let Ok(contents) = fs::read_to_string(input) {
                let ast = Ast::from_str(&contents, &Syntax::default()).unwrap();

                let mut g = Generator::new(template_m);

                if let Err(err) = g.handle(ast.nodes(), AstLevel::Top) {
                    eprint!("{}", err);
                    return;
                }
                println!("{}", g.render());
            } else {
                eprint!(
                    "Should have been able to read the file \"{}\"",
                    input.as_path().display()
                );
            }
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
