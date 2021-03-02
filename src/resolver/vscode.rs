use crate::location::Location;
use crate::resolver::{ResolvedConfig, Resolver};
use simple_error::SimpleError;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use subprocess::{Exec, Redirection};
use tempfile::tempfile;
use which::which;

const SETTING_BASE_DIR: &str = "~/.config/Code/User";

const SETTING_LOCATIONS: &'static [&'static str] = &["settings.json", "keybindings.json"];

const DYNAMIC_SETTING_COMMAND: &'static [(&'static str, &'static str)] =
    &[("extensions", "uname -a")];

pub struct VsCode {}

impl VsCode {
    pub fn new() -> VsCode {
        Self {}
    }
}

impl Resolver for VsCode {
    fn resolve_config(&self) -> Result<ResolvedConfig, Box<dyn Error>> {
        let mut files: Vec<(&'static str, File)> = vec![];
        let installed = match which::which("code") {
            Ok(_) => true,
            Err(_) => false,
        };
        // Check all the file that supposed to exist in config
        for filename in SETTING_LOCATIONS {
            match File::open(
                Path::new(shellexpand::tilde(SETTING_BASE_DIR).as_ref()).join(filename),
            ) {
                Ok(file) => {
                    files.push((filename, file));
                }
                Err(_) => {
                    return Err(Box::from(SimpleError::new(format!(
                        "Error when opening file: {}",
                        filename
                    ))));
                }
            }
        }
        // Run the dynamic generated settings and save it to tempfile
        for (name, command) in DYNAMIC_SETTING_COMMAND {
            let output = Exec::shell(command)
                .stdout(Redirection::Pipe)
                .capture()?
                .stdout_str();
            let mut temp_file = tempfile()?;
            temp_file.write_all(output.as_bytes());

            // Return file to be beginning for later read operation
            temp_file.seek(SeekFrom::Start(0)).unwrap();
            files.push((name, temp_file));
        }
        Ok(ResolvedConfig { files, installed })
    }

    fn restore_config(&self, files: Vec<(&'static str, File)>) -> Result<bool, Box<dyn Error>> {
        unimplemented!()
    }
}
