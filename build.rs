#[macro_use]
extern crate clap;

use std::env;

use clap_complete::{generate_to, shells::Bash, shells::Fish, shells::Zsh};

include!("src/cli.rs");

fn main() -> Result<(), std::io::Error> {
    let outdir = env!("CARGO_MANIFEST_DIR");
    let name = crate_name!();
    let mut app = build_cli(name, crate_version!());
    generate_to(Bash, &mut app, name, &outdir)?;
    generate_to(Zsh, &mut app, name, &outdir)?;
    generate_to(Fish, &mut app, name, &outdir)?;

    Ok(())
}
