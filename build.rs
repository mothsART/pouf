#[macro_use]
extern crate clap;

use std::env;

use clap_complete::{generate_to, shells::Bash, shells::Fish, shells::Zsh};

include!("src/cli.rs");

fn main() -> Result<(), std::io::Error> {
    let outdir = env!("CARGO_MANIFEST_DIR");
    let name = crate_name!();
    let mut cmd = build_cli(name, crate_version!());
    generate_to(Bash, &mut cmd, name, outdir)?;
    generate_to(Zsh, &mut cmd, name, outdir)?;
    generate_to(Fish, &mut cmd, name, outdir)?;

    Ok(())
}
