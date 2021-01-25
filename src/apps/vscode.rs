use crate::apps::App;
use crate::location::Location;
use std::error::Error;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use which::which;

const SETTING_BASE_DIR: &str = "~/.config";

const SETTING_LOCATIONS: &'static [&'static str] =
    &["Code/User/settings.json", "Code/User/keybindings.json"];

pub struct VsCode {}

impl VsCode {
    pub fn new() -> VsCode {
        Self {}
    }
}

impl App for VsCode {
    fn installed() -> bool {
        match which::which("code") {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn restore(base_dir: Option<String>, backup_location: PathBuf) -> Result<bool, &'static str> {
        unimplemented!()
    }

    fn backup(base_dir: Option<String>) -> Result<PathBuf, &'static str> {
        match NamedTempFile::new() {
            Ok(backup_file) => {
                // Adding files to the temp file
                let file_path = PathBuf::from(backup_file.path());
                let mut zip_writer = zip::ZipWriter::new(backup_file);

                Ok(file_path)
            }
            Err(_) => Err("Error occurred when creating temp file"),
        }
    }
}
