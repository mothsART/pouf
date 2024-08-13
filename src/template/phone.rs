use fake::Fake;
use clap::ArgMatches;
use fake::faker::phone_number::raw::{CellNumber, PhoneNumber};
use serde::{Deserialize, Serialize};

use crate::lang_env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Phone {
    pub fix: String,
    pub cell: String,
}

impl Phone {
    pub fn create(arg: &ArgMatches) -> Phone {
        Phone {
            fix: lang_return!(PhoneNumber, arg),
            cell: lang_return!(CellNumber, arg),
        }
    }
}

create_get_property!(
    Phone,
    fix: String,
    cell: String
);
