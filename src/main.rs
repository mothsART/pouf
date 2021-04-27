#[macro_use]
extern crate clap;
extern crate fake;
extern crate chrono;
extern crate http;
extern crate semver;

use std::env;
use chrono::Utc;

use fake::{Fake, Faker};
use fake::locales::{EN};
use fake::locales::{FR_FR};

use fake::faker::http::raw::*;
//use fake::faker::filesystem::raw::{Semver, SemverStable, SemverUnstable};

use fake::faker::internet::raw::{FreeEmail};
use fake::faker::administrative::raw::HealthInsuranceCode;
use fake::faker::chrono::raw::*;
use fake::faker::filesystem::raw::{MimeType};
use fake::faker::finance::raw::{Bic}; //, Iban};


macro_rules! lang_struct {
    ($struct_name:ident) => {
        match env::var("LANG") {
            Ok(_l) => {
                let split = _l.split(".");
                let vec = split.collect::<Vec<&str>>();
                lang_struct!(FreeEmail, &*vec[0].to_lowercase());
            },
            Err(_e) => {
                let val: String = $struct_name(EN).fake();
                println!("{}", val);
            }
        }
    };
    ($struct_name:ident, $lang:expr) => {
        let val: String;
        match $lang {
            "fr" | "fr_fr" => {
                val = $struct_name(FR_FR).fake();
            },
            _ => {
                val = $struct_name(EN).fake();
            }
        }
        println!("{}", val);
    };
}

fn main() {
    let matches = clap_app!(contact =>
        (version: "0.1")
        (author: "Jérémie Ferry <ferryjeremie@free.fr>")
        (about: "give fake datas")
        (@arg lang: -l --lang "precise langage")
        (@subcommand mail => 
            (about: "get mail")
            (@arg lang: -l --lang +takes_value "Lang")
        )
        (@subcommand bic => (about: "get bic"))
        (@subcommand mimetype => (about: "get mimetype"))
        (@subcommand http => (about: "get http error code"))
        (@subcommand time => (about: "get time"))
        (@subcommand date => (about: "get date"))
        (@subcommand numsecu => (about: "get numéro sécu"))
        //(@subcommand iban => (about: "get iban"))
        //(@subcommand semver => (about: "get semver"))
        
    ).get_matches();

    if let Some(mail) = matches.subcommand_matches("mail") {
        match mail.args.get("lang") {
            Some(lang) => {
                lang_struct!(FreeEmail, &*format!("{:?}", lang.vals[0]));
            },
            None => {
                lang_struct!(FreeEmail);
            }
        }
    }

    if let Some(_) = matches.subcommand_matches("bic") {
        let val: String = Bic(EN).fake();
        println!("{}", val);
    }

    if let Some(_) = matches.subcommand_matches("mimetype") {
        let val: String = MimeType(EN).fake();
        println!("{}", val);
    }

    if let Some(_) = matches.subcommand_matches("http") {
        let val: String = RfcStatusCode(EN).fake();
        println!("{}", val);
    }

    if let Some(_) = matches.subcommand_matches("numsecu") {
        let val: String = HealthInsuranceCode(FR_FR).fake();
        println!("{}", val);
    }

    if let Some(_) = matches.subcommand_matches("date") {
        let val: chrono::DateTime<Utc> = DateTime(EN).fake();
        println!("{}", val);
    }

    if let Some(_) = matches.subcommand_matches("time") {
        let val: String = Time(EN).fake();
        println!("{}", val);
    }

/*
    match matches.subcommand_matches("semver") {
        Some(_v) => {
            let val_one: String = Semver(EN).fake(); // return X.Y.Z or X-Y-Z-V.W (V equals "rc", "beta" or "alpha")
            println!("{}", val_one);

            let val_two: String = SemverStable(EN).fake(); // return X.Y.Z
            println!("{}", val_two);

            let val_three: String = SemverUnstable(EN).fake(); // return X-Y-Z-V.W
            println!("{}", val_three);
            
            let val_four: semver::Version = Faker.fake();
            println!("{}", val_four);
        },
        None => { }
    }

    if let Some(_) = matches.subcommand_matches("iban") {
        let val: String = Iban(EN).fake();
        println!("{}", val);
    }
*/
}
