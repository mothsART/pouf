use std::env;
use std::fs::File;
use std::io::prelude::*;

use clap_generate::{
    generate_to,
    generators::Bash,
    generators::Zsh
};

include!("src/cli.rs");

fn main() -> Result<(), std::io::Error> {
    let outdir = env!("CARGO_MANIFEST_DIR");
    let mut app = build_cli();
    generate_to::<Bash, _, _>(&mut app, "pouf", &outdir)?;
    generate_to::<Zsh, _, _>(&mut app, "pouf", &outdir)?;

    // Zsh correction
    let mut read_file = File::open("_pouf")?;
    let mut contents = String::new();
    read_file.read_to_string(&mut contents)?;
    let new_content = contents
        .replace("_pouf \"$@\"", "")
        .replace("#compdef pouf", "#compdef _pouf pouf");
    let mut write_file = File::create("_pouf")?;
    write_file.write_all(new_content.as_bytes())?;

    Ok(())
}
