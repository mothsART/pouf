use std::ops::Add;

use serde::{Deserialize, Serialize};

use crate::fake::Fake;
use clap::ArgMatches;
use fake::faker::address::raw::{
    CityName, CountryName, StateName, TimeZone, ZipCode,
};
use fake::locales::EN;

use crate::lang_env;
use crate::template::coordinates::Coordinate;

#[derive(Serialize, Deserialize, Debug)]
pub struct Timezone {
    pub description: String,
}

impl Timezone {
    pub fn create(_arg: &ArgMatches) -> Timezone {
        Timezone {
            description: TimeZone(EN).fake::<String>(),
        }
    }
}

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
