extern crate clap;
extern crate fake;
extern crate chrono;
extern crate http;
extern crate semver;

use std::env;
use chrono::Utc;

use clap::{Arg, App};

use fake::{Fake, Faker};
use fake::locales::{EN};
use fake::locales::{FR_FR};

//use fake::faker::filesystem::raw::{Semver, SemverStable, SemverUnstable};

const VERSION: &'static str = "0.0.1";
const AUTHOR: &'static str = "Ferry Jérémie";
const AUTHOR_MAIL: &'static str = "ferryjeremie@free.fr";

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
    /*
    let matches = clap_app!(contact =>
        //(@subcommand iban => (about: "get iban"))
        //(@subcommand semver => (about: "get semver"))
        
    ).get_matches();
    */
    let matches = App::new("pouf")
    .version(VERSION)
    .author(&*format!("{} <{}>", AUTHOR, AUTHOR_MAIL))
    .about("give fake datas")
    .subcommand(App::new("lorem.word")
        .about("give a fake word")
    )
    .subcommand(App::new("internet.mail")
        .about("give a fake mail")
        .arg(Arg::with_name("lang")
            .short("l")
            .help("give lang (ie: fr_FR)")
            .takes_value(true)
        )
        .arg(Arg::with_name("number")
            .short("n")
            .help("number of values")
            .takes_value(true)
        )
    )
    .subcommand(App::new("internet.ip")
        .about("give a fake IP (Internet Protocol)")
        .arg(Arg::with_name("ipv4")
            .short("4")
            .long("ipv4")
            .help("give exclusivly IPv4")
        )
        .arg(Arg::with_name("ipv6")
            .short("6")
            .long("ipv6")
            .help("give exclusivly IPv6")
        )
    )
    .subcommand(App::new("internet.mac")
        .about("give a fake mac adress")
    )
    .subcommand(App::new("internet.useragent")
        .about("give a fake user agent")
    )
    .subcommand(App::new("http.code")
        .about("give a fake HTTP code")
    )
    .subcommand(App::new("time.time")
        .about("give a fake time")
    )
    .subcommand(App::new("time.date")
        .about("give a fake date")
    )
    .subcommand(App::new("filesystem.mimetype")
        .about("give a fake mime-type")
    )
    .subcommand(App::new("administrative.healthinsurrancecode")
        .about("give a Health insurrance code")
    )
    .subcommand(App::new("finance.bic")
        .about("give a fake BIC (Business Identifier Code)")
    )
    .get_matches();

    if let Some(_) = matches.subcommand_matches("lorem.word") {
        use fake::faker::lorem::raw::Word;

        let val: String = Word(EN).fake();
        println!("{}", val);
    }

    if let Some(mail) = matches.subcommand_matches("internet.mail") {
        use fake::faker::internet::raw::FreeEmail;

        match mail.args.get("lang") {
            Some(lang) => {
                lang_struct!(FreeEmail, &*format!("{:?}", lang.vals[0]));
            },
            None => {
                lang_struct!(FreeEmail);
            }
        }
    }

    if let Some(ip) = matches.subcommand_matches("internet.ip") {
        use fake::faker::internet::raw::{IPv4, IPv6};

        if let Some(_) = ip.args.get("ipv4") {
            let val: String = IPv4(EN).fake();
            println!("{}", val);
            return;
        }
        if let Some(_) = ip.args.get("ipv6") {
            let val: String = IPv6(EN).fake();
            println!("{}", val);
            return;
        }
    }

    if let Some(_) = matches.subcommand_matches("internet.mac") {
        use fake::faker::internet::raw::MACAddress;

        let val: String = MACAddress(EN).fake();
        println!("{}", val);
    }

    if let Some(_) = matches.subcommand_matches("internet.useragent") {
        use fake::faker::internet::raw::UserAgent;

        let val: String = UserAgent(EN).fake();
        println!("{}", val);
    }

    if let Some(_) = matches.subcommand_matches("finance.bic") {
        use fake::faker::finance::raw::Bic;

        let val: String = Bic(EN).fake();
        println!("{}", val);
    }

    if let Some(_) = matches.subcommand_matches("filesystem.mimetype") {
        use fake::faker::filesystem::raw::MimeType;

        let val: String = MimeType(EN).fake();
        println!("{}", val);
    }

    if let Some(_) = matches.subcommand_matches("http.code") {
        use fake::faker::http::raw::RfcStatusCode;

        let val: String = RfcStatusCode(EN).fake();
        println!("{}", val);
    }

    if let Some(_) = matches.subcommand_matches("administrative.healthinsurrancecode") {
        use fake::faker::administrative::raw::HealthInsuranceCode;

        let val: String = HealthInsuranceCode(FR_FR).fake();
        println!("{}", val);
    }

    if let Some(_) = matches.subcommand_matches("time.date") {
        use fake::faker::chrono::raw::DateTime;

        let val: chrono::DateTime<Utc> = DateTime(EN).fake();
        println!("{}", val);
    }

    if let Some(_) = matches.subcommand_matches("time.time") {
        use fake::faker::chrono::raw::Time;

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
