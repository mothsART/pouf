use crate::fake::Fake;
use clap::ArgMatches;
use fake::faker::address::raw::{
    CityName, CountryName, Geohash, Latitude, Longitude, StateName, TimeZone, ZipCode,
};
use fake::locales::EN;
use serde::{Deserialize, Serialize};

use crate::lang_env;

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
pub struct Coordinate {
    pub latitude: String,
    pub longitude: String,
    pub geohash: String,
}

impl Coordinate {
    pub fn create(arg: &ArgMatches) -> Coordinate {
        Coordinate {
            latitude: lang_return!(Latitude, arg),
            longitude: lang_return!(Longitude, arg),
            geohash: Geohash(EN, 11).fake::<String>(),
        }
    }
}

create_get_property!(
    Coordinate,
    latitude: String,
    longitude: String,
    geohash: String
);

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
