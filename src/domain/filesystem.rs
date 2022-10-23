use clap::ArgMatches;
use fake::Fake;

pub fn run(matches: &ArgMatches) {
    if let Some(mimetype) = matches.subcommand_matches("filesystem.mimetype") {
        use fake::faker::filesystem::raw::MimeType;
        return each!(MimeType, mimetype);
    }

    if let Some(s) = matches.subcommand_matches("filesystem.semver") {
        use fake::faker::filesystem::raw::{Semver, SemverStable, SemverUnstable};

        if let (Some(stable), Some(unstable)) = (s.get_one::<bool>("stable"), s.get_one::<bool>("unstable")) {
            if *stable && *unstable {
                return each!(Semver, s);
            }
            if *stable {
                return each!(SemverStable, s);
            }
            if *unstable {
                return each!(SemverUnstable, s)
            }
            return each!(Semver, s);
        }
    }
}
