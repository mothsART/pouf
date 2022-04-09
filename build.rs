use std::env;

use clap_complete::{
    generate_to,
    shells::Bash,
    shells::Zsh,
    shells::Fish
};

include!("src/cli.rs");

fn main() -> Result<(), std::io::Error> {
    let outdir = env!("CARGO_MANIFEST_DIR");
    let mut app = build_cli();
    generate_to(Bash, &mut app, "pouf", &outdir)?;
    generate_to(Zsh, &mut app, "pouf", &outdir)?;
    generate_to(Fish, &mut app, "pouf", &outdir)?;

    Ok(())
}
