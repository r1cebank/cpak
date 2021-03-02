use crate::resolver::Resolver;
use std::path::PathBuf;

mod git;

pub trait Location {
    fn list_backups() -> Result<Vec<String>, &'static str>;
    fn upload(backup_dir: PathBuf) -> Result<bool, &'static str>;
    fn download(backup_dir: PathBuf, id: String) -> Result<PathBuf, &'static str>;
}
