use std::env;
use clap_generate::{
    generate_to,
    generators::Bash,
    generators::Zsh
};

include!("src/cli.rs");

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };
    let mut app = build_cli();
    generate_to::<Bash, _, _>(&mut app, "pouf", &outdir);
    generate_to::<Zsh, _, _>(&mut app, "pouf", &outdir);
}
