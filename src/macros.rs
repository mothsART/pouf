macro_rules! lang_struct_return {
    ($struct_name:ident) => {
        match lang_env() {
            Some(lang) => {
                lang_struct_return!($struct_name, &*lang)
            }
            None => {
                let val: String = $struct_name(fake::locales::EN).fake();
                format!("{}", val)
            }
        }
    };
    ($struct_name:ident, $lang:expr) => {
        match $lang {
            "fr" | "fr_fr" => {
                format!("{}", $struct_name(fake::locales::FR_FR).fake::<String>())
            }
            _ => {
                format!("{}", $struct_name(fake::locales::EN).fake::<String>())
            }
        }
    };
}

macro_rules! lang_return {
    ($struct_name:ident, $matches:expr) => {
        match $matches.try_contains_id("lang") {
            Ok(_) => match $matches.get_one::<String>("lang") {
                Some(lang) => {
                    lang_struct_return!($struct_name, lang.as_str())
                }
                None => {
                    lang_struct_return!($struct_name)
                }
            },
            Err(_) => {
                lang_struct_return!($struct_name)
            }
        }
    };
}

macro_rules! lang_struct {
    ($struct_name:ident) => {
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
        match $matches.try_contains_id("lang") {
            Ok(_) => match $matches.get_one::<String>("lang") {
                Some(lang) => {
                    lang_struct!($struct_name, lang.as_str());
                }
                None => {
                    lang_struct!($struct_name);
                }
            },
            Err(_) => {
                lang_struct!($struct_name);
            }
        }
    };
}

macro_rules! each {
    ($struct_name:ident, $matches:expr) => {
        match $matches.get_one::<String>("number").map(|s| s.as_str()) {
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
    ($struct_name:ident, $force_lang:ident, $has_user_lang:expr) => {
        use clap::error::ErrorKind;
        use clap::{Command, Error};

        let val: String;

        if !$has_user_lang {
            val = $struct_name($force_lang).fake();
            println!("{}", val);
        } else {
            match lang_env() {
                Some(lang) => {
                    force_lang_struct!($struct_name, $force_lang, &*lang, $has_user_lang);
                }
                None => {
                    match stringify!($force_lang) {
                        "FR" | "FR_FR" | "EN" => {
                            val = $struct_name($force_lang).fake();
                        }
                        other_lang => {
                            Error::raw(
                                ErrorKind::InvalidSubcommand,
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
        }
    };
    ($struct_name:ident, $force_lang:ident, $lang:expr, $has_user_lang:expr) => {
        use clap::error::ErrorKind;
        use clap::{Command, Error};

        let val: String;

        match $lang {
            "fr" | "fr_fr" => match stringify!($force_lang) {
                "FR" | "FR_FR" => {
                    val = $struct_name($force_lang).fake();
                }
                _ => {
                    Error::raw(
                        ErrorKind::InvalidSubcommand,
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
                        ErrorKind::InvalidSubcommand,
                        "this feature is not supported in English.",
                    )
                    .format(&mut Command::new(""))
                    .exit();
                }
            },
            other_lang => {
                Error::raw(
                    ErrorKind::InvalidSubcommand,
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
        match $matches.try_contains_id("lang") {
            Ok(_) => match $matches.get_one::<String>("lang").map(|s| s.as_str()) {
                Some(lang) => {
                    force_lang_struct!($struct_name, $force_lang, lang, true);
                }
                None => {
                    force_lang_struct!($struct_name, $force_lang, true);
                }
            },
            Err(_) => {
                force_lang_struct!($struct_name, $force_lang, false);
            }
        }
    };
}

macro_rules! force_each {
    ($struct_name:ident, $force_lang:ident, $matches:expr) => {
        match $matches.get_one::<String>("number").map(|s| s.as_str()) {
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

macro_rules! create_get_property {
    ($struct_name:ident, $( $field_name:ident : $field_type:ty ),* ) => {
        use std::any::Any;
        use crate::template::template_trait::TemplateObject;

        impl TemplateObject for $struct_name {
            fn get_property(&self, key: &str) -> Option<&dyn Any> {
                match key {
                    $(
                        stringify!($field_name) => Some(&self.$field_name as &dyn Any),
                    )*
                    _ => None,
                }
            }
        }
    }
}
