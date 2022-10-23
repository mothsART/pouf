use clap::ArgMatches;
use fake::Fake;

pub fn run(matches: &ArgMatches) {
    if let Some(mimetype) = matches.subcommand_matches("filesystem.mimetype") {
        use fake::faker::filesystem::raw::MimeType;
        return each!(MimeType, mimetype);
    }

    if let Some(s) = matches.subcommand_matches("filesystem.semver") {
        use fake::faker::filesystem::raw::{Semver, SemverStable, SemverUnstable};

        if !s.args_present() || (s.contains_id("stable") && s.contains_id("unstable")) {
            return each!(Semver, s);
        }

        if s.contains_id("stable") {
            return each!(SemverStable, s);
        }
        if s.contains_id("unstable") {
            each!(SemverUnstable, s)
        }
    }
}
