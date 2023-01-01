use crate::fake::Fake;
use clap::ArgMatches;
use fake::faker::address::raw::{CityName, CountryName, Latitude, Longitude, StateName, ZipCode};
use serde::{Deserialize, Serialize};

use crate::lang_env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinate {
    pub latitude: String,
    pub longitude: String,
}

impl Coordinate {
    pub fn create(arg: &ArgMatches) -> Coordinate {
        Coordinate {
            latitude: lang_return!(Latitude, arg),
            longitude: lang_return!(Longitude, arg),
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
}

impl Address {
    pub fn create(arg: &ArgMatches) -> Address {
        Address {
            city: lang_return!(CityName, arg),
            state: lang_return!(StateName, arg),
            country: lang_return!(CountryName, arg),
            zipcode: lang_return!(ZipCode, arg),
            coordinates: Coordinate::create(arg),
        }
    }
}
