use serde::{Deserialize, Serialize};

use crate::fake::Fake;
use clap::ArgMatches;
use fake::faker::address::raw::{Geohash, Latitude, Longitude};
use fake::locales::EN;

use crate::lang_env;

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
