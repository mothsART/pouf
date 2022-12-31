#[macro_use]
extern crate clap;

extern crate chrono;
extern crate fake;
extern crate http;
extern crate semver;
extern crate serde;
extern crate tera;

use std::env;
use std::fs;
use std::path::PathBuf;

use tera::{Context, Tera};

mod cli;
#[macro_use]
mod macros;
mod domain;
mod template;

use crate::template::{parser::parse, people::People};

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
            let mut contents =
                fs::read_to_string(input).expect("Should have been able to read the file");

            let nodes = parse(&contents);

            let mut nb_peoples = 0;
            for n in nodes {
                match n.key.as_str() {
                    "peoples_nb" => {
                        nb_peoples = n.value.parse::<i32>().unwrap_or(0);
                        contents = contents.replace(&format!("{}\n", &n.tag), "");
                    }
                    "lang" => {
                        contents = contents.replace(&format!("{}\n", &n.tag), "");
                    }
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
            context.insert("peoples", &peoples);
            context.insert("people", &People::create(template_m));

            let result = Tera::one_off(&contents, &context, true);

            println!("{}", result.unwrap());
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
