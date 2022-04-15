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
