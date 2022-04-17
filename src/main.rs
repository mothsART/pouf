#[macro_use]
extern crate clap;

extern crate fake;
extern crate chrono;
extern crate http;
extern crate semver;

use std::env;

mod cli;
#[macro_use]
mod macros;
mod domain;

fn lang_env() -> Option<String> {
    match std::env::var("LANG") {
        Ok(_l) => {
            match _l.find(".") {
                Some(pos) => {
                    Some(_l[0..pos].to_lowercase().to_string())
                },
                None => {
                    None
                }
            }
        },
        Err(_) => {
            None
        }
    }
}

fn main() {
    let matches = cli::build_cli(crate_name!(), crate_version!()).get_matches();

    domain::administrative::run(&matches);
    domain::auto::run(&matches);
    domain::barecode::run(&matches);
    domain::filesystem::run(&matches);
    domain::finance::run(&matches);
    domain::http::run(&matches);
    domain::internet::run(&matches);
    domain::lorem::run(&matches);
    domain::people::run(&matches);
    domain::time::run(&matches);
}
