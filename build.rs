use std::env;

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

    Ok(())
}
