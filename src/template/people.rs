use crate::fake::Fake;
use clap::ArgMatches;
use fake::faker::name::raw::{FirstName, LastName, Title};
use serde::{Deserialize, Serialize};

use crate::lang_env;
use crate::template::address::Address;

#[derive(Serialize, Deserialize, Debug)]
pub struct People {
    pub title: String,
    pub first_name: String,
    pub last_name: String,

    pub location: Address,
}

impl People {
    pub fn create(arg: &ArgMatches) -> People {
        People {
            title: lang_return!(Title, arg),
            first_name: lang_return!(FirstName, arg),
            last_name: lang_return!(LastName, arg),

            location: Address::create(arg),
        }
    }
}
