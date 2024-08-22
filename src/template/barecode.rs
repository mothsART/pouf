use fake::Fake;
use clap::ArgMatches;
use fake::faker::barcode::raw::{Isbn, Isbn10, Isbn13};
use fake::locales::EN;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BareCode {
    pub isbn: String,
    pub isbn10: String,
    pub isbn13: String,
}

impl BareCode {
    pub fn create(_arg: &ArgMatches) -> BareCode {
        BareCode {
            isbn: Isbn(EN).fake(),
            isbn10: Isbn10(EN).fake(),
            isbn13: Isbn13(EN).fake(),
        }
    }
}

create_get_property!(
    BareCode,
    isbn: String,
    isbn10: String,
    isbn13: String
);
