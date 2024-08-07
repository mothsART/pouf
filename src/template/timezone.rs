use serde::{Deserialize, Serialize};

use crate::fake::Fake;
use clap::ArgMatches;
use fake::faker::address::raw::TimeZone;
use fake::locales::EN;

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

create_get_property!(
    Timezone,
    description: String
);
