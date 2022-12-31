use crate::fake::Fake;
use clap::ArgMatches;
use fake::faker::address::raw::{CityName, CountryName, StateName, ZipCode};
use serde::{Deserialize, Serialize};

use crate::lang_env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub city: String,
    pub state: String,
    pub country: String,
    pub zipcode: String,
}

impl Address {
    pub fn create(arg: &ArgMatches) -> Address {
        Address {
            city: lang_return!(CityName, arg),
            state: lang_return!(StateName, arg),
            country: lang_return!(CountryName, arg),
            zipcode: lang_return!(ZipCode, arg),
        }
    }
}
