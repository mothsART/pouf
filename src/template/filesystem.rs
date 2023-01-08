use crate::fake::Fake;
use clap::ArgMatches;
use fake::faker::filesystem::raw::{
    DirPath,
    FileExtension,
    FileName,
    FilePath,
    MimeType,
    Semver,
};
use fake::locales::EN;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileSystem {
    pub dir_path: String,
    pub file_extension: String,
    pub file_name: String,
    pub file_path: String,
    pub mime_type: String,

    pub semver: String,
}

impl FileSystem {
    pub fn create(_arg: &ArgMatches) -> FileSystem {
        FileSystem {
            dir_path: DirPath(EN).fake::<String>(),
            file_extension: FileExtension(EN).fake::<String>(),
            file_name: FileName(EN).fake::<String>(),
            file_path: FilePath(EN).fake::<String>(),
            mime_type: MimeType(EN).fake::<String>(),

            semver: Semver(EN).fake::<String>(),
        }
    }
}
