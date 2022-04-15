macro_rules! lang_struct {
    ($struct_name:ident) => {
        match std::env::var("LANG") {
            Ok(_l) => {
                let split = _l.split(".");
                let vec = split.collect::<Vec<&str>>();
                lang_struct!($struct_name, &*vec[0].to_lowercase());
            },
            Err(_e) => {
                let val: String = $struct_name(fake::locales::EN).fake();
                println!("{}", val);
            }
        }
    };
    ($struct_name:ident, $lang:expr) => {
        let val: String;
        match $lang {
            "fr" | "fr_fr" => {
                val = $struct_name(fake::locales::FR_FR).fake();
            },
            _ => {
                val = $struct_name(fake::locales::EN).fake();
            }
        }
        println!("{}", val);
    };
}

macro_rules! lang {
    ($struct_name:ident, $matches:expr) => {
        match $matches.value_of("lang") {
            Some(lang) => {
                lang_struct!($struct_name, &*format!("{}", lang));
            },
            None => {
                lang_struct!($struct_name);
            }
        }
    }
}

macro_rules! each {
    ($struct_name:ident, $matches:expr) => {
        match $matches.value_of("number") {
            Some(number) => {
                let n: i32 = number.parse().unwrap_or(1);
                for _i in 0..n {
                    lang!($struct_name, $matches);
                }
            },
            None => {
                lang_struct!($struct_name);
            }
        }
    }
}
