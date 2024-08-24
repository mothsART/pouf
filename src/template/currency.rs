use clap::ArgMatches;
use fake::faker::currency::raw::{CurrencyCode, CurrencyName, CurrencySymbol};
use fake::Fake;
use serde::{Deserialize, Serialize};

use crate::lang_env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub symbol: String,
}

impl Currency {
    pub fn create(arg: &ArgMatches) -> Currency {
        Currency {
            code: lang_return!(CurrencyCode, arg),
            name: lang_return!(CurrencyName, arg),
            symbol: lang_return!(CurrencySymbol, arg),
        }
    }
}

create_get_property!(
    Currency,
    code: String,
    name: String,
    symbol: String
);
