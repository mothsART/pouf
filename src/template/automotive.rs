use crate::fake::Fake;
use clap::ArgMatches;
use fake::faker::automotive::raw::LicencePlate;
use fake::locales::FR_FR;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Automotive {
    pub licence_plate: String,
}

impl Automotive {
    pub fn create(_arg: &ArgMatches) -> Automotive {
        Automotive {
            licence_plate: LicencePlate(FR_FR).fake::<String>(),
        }
    }
}
