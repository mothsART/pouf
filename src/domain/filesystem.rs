use clap::ArgMatches;
use fake::Fake;

pub fn run(matches: &ArgMatches) {
    if let Some(mimetype) = matches.subcommand_matches("filesystem.mimetype") {
        use fake::faker::filesystem::raw::MimeType;
        return each!(MimeType, mimetype);
    }

    if let Some(s) = matches.subcommand_matches("filesystem.semver") {
        use fake::faker::filesystem::raw::{Semver, SemverStable, SemverUnstable};

        if !s.args_present() || (s.is_present("stable") && s.is_present("unstable")) {
            return each!(Semver, s);
        }

        if s.is_present("stable") {
            return each!(SemverStable, s);
        }
        if s.is_present("unstable") {
            each!(SemverUnstable, s)
        }
    }
}
