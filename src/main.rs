#[macro_use]
extern crate clap;

extern crate fake;

use fake::{Fake};
use fake::locales::{EN};
use fake::locales::{FR_FR};
use fake::faker::internet::raw::{FreeEmail};
use fake::faker::finance::raw::{Bic, Iban};

fn main() {
    let matches = clap_app!(contact =>
        (version: "0.1")
        (author: "Jérémie Ferry <ferryjeremie@free.fr>")
        (about: "give fake datas")
        (@subcommand mail => (about: "get mail"))
        (@subcommand bic => (about: "get bic"))
        (@subcommand iban => (about: "get iban"))
    ).get_matches();

    match matches.subcommand_matches("mail") {
        Some(_v) => {
             let val: String = FreeEmail(FR_FR).fake();
            println!("{:?}", val);
        },
        None => { }
    }

    match matches.subcommand_matches("bic") {
        Some(_v) => {
            let val: String = Bic(EN).fake();
            println!("{}", val);
        },
        None => { }
    }

    match matches.subcommand_matches("iban") {
        Some(_v) => {
            let val: String = Iban(EN).fake();
            println!("{}", val);
        },
        None => { }
    }
}
