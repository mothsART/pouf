macro_rules! lang_struct {
    ($struct_name:ident) => {
        use crate::lang_env;
        match lang_env() {
            Some(lang) => {
                lang_struct!($struct_name, &*lang);
            }
            None => {
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
            }
            _ => {
                val = $struct_name(fake::locales::EN).fake();
            }
        }
        println!("{}", val);
    };
}

macro_rules! lang {
    ($struct_name:ident, $matches:expr) => {
        if !$matches.is_valid_arg("lang") {
            lang_struct!($struct_name);
        } else {
            match $matches.value_of("lang") {
                Some(lang) => {
                    lang_struct!($struct_name, lang);
                }
                None => {
                    lang_struct!($struct_name);
                }
            }
        }
    };
}

macro_rules! each {
    ($struct_name:ident, $matches:expr) => {
        match $matches.value_of("number") {
            Some(number) => {
                let n: i32 = number.parse().unwrap_or(1);
                for _i in 0..n {
                    lang!($struct_name, $matches);
                }
            }
            None => {
                lang!($struct_name, $matches);
            }
        }
    };
}

macro_rules! force_lang_struct {
    ($struct_name:ident, $force_lang:ident) => {
        use crate::lang_env;
        use clap::{Command, Error, ErrorKind};

        let val: String;
        match lang_env() {
            Some(lang) => {
                force_lang_struct!($struct_name, $force_lang, &*lang);
            }
            None => {
                match stringify!($force_lang) {
                    "FR" | "FR_FR" | "EN" => {
                        val = $struct_name($force_lang).fake();
                    }
                    other_lang => {
                        Error::raw(
                            ErrorKind::UnrecognizedSubcommand,
                            &*format!(
                                "this feature is not supported in \"{}\" langage.",
                                other_lang
                            ),
                        )
                        .format(&mut Command::new(""))
                        .exit();
                    }
                }
                println!("{}", val);
            }
        }
    };
    ($struct_name:ident, $force_lang:ident, $lang:expr) => {
        use clap::{Command, Error, ErrorKind};

        let val: String;

        match $lang {
            "fr" | "fr_fr" => match stringify!($force_lang) {
                "FR" | "FR_FR" => {
                    val = $struct_name($force_lang).fake();
                }
                _ => {
                    Error::raw(
                        ErrorKind::UnrecognizedSubcommand,
                        "this feature is not supported in French.",
                    )
                    .format(&mut Command::new(""))
                    .exit();
                }
            },
            "en" => match stringify!($force_lang) {
                "EN" => {
                    val = $struct_name($force_lang).fake();
                }
                _ => {
                    Error::raw(
                        ErrorKind::UnrecognizedSubcommand,
                        "this feature is not supported in English.",
                    )
                    .format(&mut Command::new(""))
                    .exit();
                }
            },
            other_lang => {
                Error::raw(
                    ErrorKind::UnrecognizedSubcommand,
                    &*format!(
                        "this feature is not supported in \"{}\" langage.",
                        other_lang
                    ),
                )
                .format(&mut Command::new(""))
                .exit();
            }
        }
        println!("{}", val);
    };
}

macro_rules! force_lang {
    ($struct_name:ident, $force_lang:ident, $matches:expr) => {
        if !$matches.is_valid_arg("lang") {
            force_lang_struct!($struct_name, $force_lang);
        } else {
            match $matches.value_of("lang") {
                Some(lang) => {
                    force_lang_struct!($struct_name, $force_lang, lang);
                }
                None => {
                    force_lang_struct!($struct_name, $force_lang);
                }
            }
        }
    };
}

macro_rules! force_each {
    ($struct_name:ident, $force_lang:ident, $matches:expr) => {
        match $matches.value_of("number") {
            Some(number) => {
                let n: i32 = number.parse().unwrap_or(1);
                for _i in 0..n {
                    force_lang!($struct_name, $force_lang, $matches);
                }
            }
            None => {
                force_lang!($struct_name, $force_lang, $matches);
            }
        }
    };
}
