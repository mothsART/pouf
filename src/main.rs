extern crate clap;
extern crate fake;
extern crate chrono;
extern crate http;
extern crate semver;

mod cli;
use std::env;
use chrono::Utc;

use fake::Fake;
use fake::locales::{EN};
use fake::locales::{FR_FR};

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
    let matches = cli::build_cli().get_matches();

    if let Some(_) = matches.subcommand_matches("lorem.word") {
        use fake::faker::lorem::raw::Word;

        let val: String = Word(EN).fake();
        println!("{}", val);
    }

    if let Some(_) = matches.subcommand_matches("barcode.isbn") {
        use fake::faker::barcode::fr_fr;
        
        let val: String = fr_fr::Isbn13().fake();
        println!("{}", val);
    }

    if let Some(mail) = matches.subcommand_matches("internet.mail") {
        use fake::faker::internet::raw::FreeEmail;

        match mail.value_of("lang") {
            Some(lang) => {
                lang_struct!(FreeEmail, &*format!("{:?}", lang));
            },
            None => {
                lang_struct!(FreeEmail);
            }
        }
    }

    if let Some(ip) = matches.subcommand_matches("internet.ip") {
        use fake::faker::internet::raw::{IPv4, IPv6};
        use fake::faker::boolean::en;

        let val: String;

        if !ip.args_present() || (ip.is_present("ipv4") && ip.is_present("ipv6")) {
            if en::Boolean(50).fake() {
                val = IPv4(EN).fake();
                println!("{}", val);
                return;
            }
            val = IPv6(EN).fake();
            println!("{}", val);
            return;
        }

        if ip.is_present("ipv4") {
            val = IPv4(EN).fake();
            println!("{}", val);
            return;
        }
        if ip.is_present("ipv6") {
            val = IPv6(EN).fake();
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

    if let Some(c) = matches.subcommand_matches("internet.color") {
        use fake::faker::color::raw::{HexColor, RgbColor, RgbaColor, HslColor, HslaColor, Color};

        if !c.args_present() {
            let all_color: String = Color(EN).fake();
            println!("{}", all_color);
            return;
        }

        if c.is_present("hexa") {
            let hex_val: String = HexColor(EN).fake();
            println!("{}", hex_val);
        }
        if c.is_present("rgb") {
            let rgb_val: String = RgbColor(EN).fake();
            println!("{}", rgb_val);
        }
        if c.is_present("rgba") {
            let rgba_val: String = RgbaColor(EN).fake();
            println!("{}", rgba_val);
        }
        if c.is_present("hsl") {
            let hsl_val: String = HslColor(EN).fake();
            println!("{}", hsl_val);
        }
        if c.is_present("hsla") {
            let hsla_val: String = HslaColor(EN).fake();
            println!("{}", hsla_val);
        }
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

    if let Some(s) = matches.subcommand_matches("filesystem.semver") {
        use fake::faker::filesystem::raw::{Semver, SemverStable, SemverUnstable};
        let val: String;

        if !s.args_present() || (s.is_present("stable") && s.is_present("unstable")) {
            val = Semver(EN).fake();
            println!("{}", val);
            return;
        }

        if s.is_present("stable") {
            val = SemverStable(EN).fake();
            println!("{}", val);
            return;
        }

        if s.is_present("unstable") {
            val = SemverUnstable(EN).fake();
            println!("{}", val);
            return;
        }
    }

    if let Some(_) = matches.subcommand_matches("auto.licenseplate") {
        use fake::faker::automotive::raw::LicencePlate;
        let val: String = LicencePlate(FR_FR).fake();
        println!("{}", val);
    }
}
