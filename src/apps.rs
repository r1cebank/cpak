use crate::location::Location;
use std::path::PathBuf;

mod vscode;

/// The app trait, defines required method to resolve config paths
pub trait App {
    /// Returns true if the application is installed
    fn installed() -> bool;
    /// Given a backup location, run the setting restore
    fn restore(base_dir: Option<String>, backup_location: PathBuf) -> Result<bool, &'static str>;
    /// Run the backup task, returns the bundled settings file path
    fn backup(base_dir: Option<String>) -> Result<PathBuf, &'static str>;
}
