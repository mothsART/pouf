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

use serde::{Deserialize, Serialize};

mod cli;
#[macro_use]
mod macros;
mod domain;
mod template;

use crate::template::color::Color;
use crate::template::lorem::Lorem;
use crate::template::{parser::parse, people::People};

fn lang_env() -> Option<String> {
    match std::env::var("LANG") {
        Ok(_l) => _l.find('.').map(|pos| _l[0..pos].to_lowercase()),
        Err(_) => None,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Accounts {
    pub key: u32,
}

impl Accounts {
    fn new() -> Accounts {
        Accounts { key: 0 }
    }
}

pub struct Account {
    pub name: String,
}

impl Iterator for Accounts {
    type Item = Account;

    fn next(&mut self) -> Option<Self::Item> {
        self.key += 1;

        if self.key > 3 {
            return None;
        }

        Some(Account {
            name: format!("generate value : {}", self.key),
        })
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
            let mut nb_lorems = 0;
            let mut lorems_words_nb = 0;
            let mut lorems_sentences_nb = 0;
            let mut lorems_paragraphs_nb = 0;
            let mut colors_nb = 0;
            for n in nodes {
                match n.key.as_str() {
                    "peoples_nb" => {
                        nb_peoples = n.value.parse::<i32>().unwrap_or(0);
                        contents = contents.replace(&format!("{}\n", &n.tag), "");
                    }
                    "lorems_nb" => {
                        nb_lorems = n.value.parse::<u32>().unwrap_or(0);
                        contents = contents.replace(&format!("{}\n", &n.tag), "");
                    }
                    "lorems_words_nb" => {
                        lorems_words_nb = n.value.parse::<u32>().unwrap_or(0);
                        contents = contents.replace(&format!("{}\n", &n.tag), "");
                    }
                    "lorems_sentences_nb" => {
                        lorems_sentences_nb = n.value.parse::<u32>().unwrap_or(0);
                        contents = contents.replace(&format!("{}\n", &n.tag), "");
                    }
                    "lorems_paragraphs_nb" => {
                        lorems_paragraphs_nb = n.value.parse::<u32>().unwrap_or(0);
                        contents = contents.replace(&format!("{}\n", &n.tag), "");
                    }
                    "colors_nb" => {
                        colors_nb = n.value.parse::<u32>().unwrap_or(0);
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

            let mut lorems = vec![];
            if nb_lorems > 0 {
                for _ in 0..nb_lorems {
                    lorems.push(Lorem::create(
                        template_m,
                        lorems_words_nb,
                        lorems_sentences_nb,
                        lorems_paragraphs_nb,
                    ));
                }
            }

            let mut colors = vec![];
            if colors_nb > 0 {
                for _ in 0..colors_nb {
                    colors.push(Color::create(template_m));
                }
            }

            let mut context = Context::new();
            context.insert("peoples", &peoples);
            context.insert("people", &People::create(template_m));
            context.insert("lorems", &lorems);
            context.insert(
                "lorem",
                &Lorem::create(
                    template_m,
                    lorems_words_nb,
                    lorems_sentences_nb,
                    lorems_paragraphs_nb,
                ),
            );
            context.insert("colors", &colors);
            context.insert("color", &Color::create(template_m));

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
