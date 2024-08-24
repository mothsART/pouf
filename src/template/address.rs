use serde::{Deserialize, Serialize};

use clap::ArgMatches;
use fake::faker::address::raw::{CityName, CountryName, StateName, ZipCode};
use fake::Fake;

use crate::lang_env;
use crate::template::coordinates::Coordinate;
use crate::template::timezone::Timezone;

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub city: String,
    pub state: String,
    pub country: String,
    pub zipcode: String,

    pub coordinates: Coordinate,

    pub timezone: Timezone,
}

impl Address {
    pub fn create(arg: &ArgMatches) -> Address {
        Address {
            city: lang_return!(CityName, arg),
            state: lang_return!(StateName, arg),
            country: lang_return!(CountryName, arg),
            zipcode: lang_return!(ZipCode, arg),

            coordinates: Coordinate::create(arg),

            timezone: Timezone::create(arg),
        }
    }
}

create_get_property!(
    Address,
    city: String,
    state: String,
    country: String,
    zipcode: String
);
