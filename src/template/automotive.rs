use clap::ArgMatches;
use fake::faker::automotive::raw::LicencePlate;
use fake::locales::FR_FR;
use fake::Fake;
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

create_get_property!(
    Automotive,
    licence_plate: String
);
