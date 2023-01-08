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

use crate::template::{
    phone::Phone,
    internet::Internet,
    automotive::Automotive,
    color::Color,
    lorem::Lorem,
    barecode::BareCode,
    parser::parse,
    people::People,
    filesystem::FileSystem,
    job::Job,
    currency::Currency,
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
            let mut contents =
                fs::read_to_string(input).expect("Should have been able to read the file");

            let nodes = parse(&contents);

            let mut nb_peoples = 0;
            let mut nb_lorems = 0;
            let mut lorems_words_nb = 0;
            let mut lorems_sentences_nb = 0;
            let mut lorems_paragraphs_nb = 0;
            let mut colors_nb = 0;
            let mut barecodes_nb = 0;
            let mut autos_nb = 0;
            let mut internets_nb = 0;
            let mut phones_nb = 0;
            let mut filesystems_nb = 0;
            let mut jobs_nb = 0;
            let mut currencies_nb = 0;

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
                    "barecodes_nb" => {
                        barecodes_nb = n.value.parse::<u32>().unwrap_or(0);
                        contents = contents.replace(&format!("{}\n", &n.tag), "");
                    }
                    "autos_nb" => {
                        autos_nb = n.value.parse::<u32>().unwrap_or(0);
                        contents = contents.replace(&format!("{}\n", &n.tag), "");
                    }
                    "internets_nb" => {
                        internets_nb = n.value.parse::<u32>().unwrap_or(0);
                        contents = contents.replace(&format!("{}\n", &n.tag), "");
                    }
                    "phones_nb" => {
                        phones_nb = n.value.parse::<u32>().unwrap_or(0);
                        contents = contents.replace(&format!("{}\n", &n.tag), "");
                    }
                    "filesystems_nb" => {
                        filesystems_nb = n.value.parse::<u32>().unwrap_or(0);
                        contents = contents.replace(&format!("{}\n", &n.tag), "");
                    }
                    "jobs_nb" => {
                        jobs_nb = n.value.parse::<u32>().unwrap_or(0);
                        contents = contents.replace(&format!("{}\n", &n.tag), "");
                    }
                    "currencies_nb" => {
                        currencies_nb = n.value.parse::<u32>().unwrap_or(0);
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

            let mut barecodes = vec![];
            if barecodes_nb > 0 {
                for _ in 0..barecodes_nb {
                    barecodes.push(Color::create(template_m));
                }
            }

            let mut autos = vec![];
            if autos_nb > 0 {
                for _ in 0..autos_nb {
                    autos.push(Automotive::create(template_m));
                }
            }

            let mut internets = vec![];
            if internets_nb > 0 {
                for _ in 0..internets_nb {
                    internets.push(Internet::create(template_m));
                }
            }

            let mut phones = vec![];
            if phones_nb > 0 {
                for _ in 0..phones_nb {
                    phones.push(Phone::create(template_m));
                }
            }

            let mut filesystems = vec![];
            if filesystems_nb > 0 {
                for _ in 0..filesystems_nb {
                    filesystems.push(FileSystem::create(template_m));
                }
            }

            let mut jobs = vec![];
            if jobs_nb > 0 {
                for _ in 0..jobs_nb {
                    jobs.push(Job::create(template_m));
                }
            }

            let mut currencies = vec![];
            if currencies_nb > 0 {
                for _ in 0..jobs_nb {
                    currencies.push(Currency::create(template_m));
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
            context.insert("barecodes", &barecodes);
            context.insert("barecode", &BareCode::create(template_m));
            context.insert("autos", &autos);
            context.insert("auto", &Automotive::create(template_m));
            context.insert("internets", &internets);
            context.insert("internet", &Internet::create(template_m));
            context.insert("phones", &phones);
            context.insert("phone", &Phone::create(template_m));
            context.insert("filesystems", &filesystems);
            context.insert("filesystem", &FileSystem::create(template_m));
            context.insert("jobs", &jobs);
            context.insert("job", &Job::create(template_m));
            context.insert("currencies", &currencies);
            context.insert("currency", &Currency::create(template_m));

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
