// TODO: unfinished

use chrono::Utc;
use clonetree::{Options, clone_tree};
use std::path::Path;

pub fn main(server_data_folder: &Path, backups_folder: &Path) -> Result<(), clonetree::Error> {
    let destination = backups_folder.join(Utc::now().to_rfc3339());
    clone_tree(server_data_folder, destination, &Options::new())?;
    return Ok(());
}
