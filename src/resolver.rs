use crate::location::Location;
use simple_error::SimpleError;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

pub mod vscode;

/// The result after a resolve config run
#[derive(Debug)]
pub struct ResolvedConfig {
    pub files: Vec<(&'static str, File)>,
    pub installed: bool,
}

/// The resolver trait, defines how to resolve config for an app
pub trait Resolver {
    /// Returns true if the application is installed
    fn resolve_config(&self) -> Result<ResolvedConfig, Box<dyn Error>>;
    fn restore_config(&self, files: Vec<(&'static str, File)>) -> Result<bool, Box<dyn Error>>;
}
