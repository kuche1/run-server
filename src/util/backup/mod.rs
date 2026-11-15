// TODO: add support for keeping snapshots once a month indefinetely

// This code has been copy-pasted from another project, so not eveything might make perfect
// sense in this context

use chrono::prelude::DateTime;
use chrono::{Duration, Utc};
use clonetree::{Options, clone_tree};
use std::fs;
use std::path::PathBuf;

/// Take snapshots of a given folder
pub struct Args {
    /// Destination to place snapshots into
    pub dst: PathBuf,

    /// How many days to wait before taking a new snapshot
    // default_value_t = -0.9
    pub cooldown_days: f64,

    /// How many snapshots to keep, in addition to the latest one
    // default_value_t = 31
    pub history: u32,
}

struct Snapshot {
    folder: PathBuf,
    date: DateTime<Utc>,
}

/// `source` - source folder to copy
pub fn main(source: &str, args: Args) -> anyhow::Result<()> {
    env_logger::init();

    let source: PathBuf = source.into();
    if !source.exists() {
        anyhow::bail!("Error: Source does not exist: {}", source.display());
    }
    if !source.is_dir() {
        anyhow::bail!("Error: Source is not a folder: {}", source.display());
    }

    let now = get_current_date();

    let snapshot_folder = args.dst;
    let snapshots = get_snapshots(&snapshot_folder);
    let latest_snapshot_date = get_latest_snapshot_date(&snapshots);

    let cooldown_seconds = args.cooldown_days * 24.0 * 60.0 * 60.0;
    let next_snapshot_date =
        latest_snapshot_date + Duration::seconds(cooldown_seconds.round() as i64);

    if next_snapshot_date > now {
        println!("Not yet time to take a snapshot");
        return Ok(());
    }

    println!("Time to take a snapshot");

    let destination = snapshot_folder.join(now.to_rfc3339());

    println!("Cloning to: {}", destination.display());

    // This is going to create the destination subfolders if they do not exist
    clone_tree(source, destination, &Options::new())?;

    // We will only ever attempt to delete any of the snapshots if the last backup
    // has been successful.
    // The last snapshot will NOT be counted - so that if you set the history max
    // to 0 the latest snapshot won't be instantly deleted.
    delete_old_snapshots(snapshots, args.history);

    Ok(())
}

fn get_current_date() -> DateTime<Utc> {
    return Utc::now();
}

/// The returned snapshots will be sorted.
/// Index 0 will be the oldest snapshot.
fn get_snapshots(snapshot_folder: &PathBuf) -> Vec<Snapshot> {
    let mut snapshots = vec![];

    let iter = match fs::read_dir(snapshot_folder) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Could not read dir: {}: {}", e, snapshot_folder.display());
            return vec![];
        }
    };

    for entry in iter {
        let entry = match entry {
            Ok(v) => v,
            Err(e) => {
                log::error!("Could not check entry: {}", e);
                continue;
            }
        };

        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let folder_name = match path.file_name() {
            Some(v) => v,
            None => {
                log::error!("[Impossible] Relative path: {}", path.display());
                continue;
            }
        };

        let folder_name = match folder_name.to_str() {
            Some(v) => v,
            None => {
                log::error!("[Impossible] Not a valid UTF-8: {}", folder_name.display());
                continue;
            }
        };

        let folder_date: DateTime<Utc> = match folder_name.parse() {
            Ok(v) => v,
            Err(_e) => {
                log::error!("Folder name is not a valid date: {}", path.display());
                continue;
            }
        };

        snapshots.push(Snapshot {
            folder: path,
            date: folder_date,
        });
    }

    snapshots.sort_by(|a, b| a.date.cmp(&b.date));

    return snapshots;
}

fn get_latest_snapshot_date(snapshots: &Vec<Snapshot>) -> DateTime<Utc> {
    if snapshots.len() == 0 {
        return DateTime::<Utc>::MIN_UTC;
    }

    return snapshots[snapshots.len() - 1].date;

    // let mut latest_snapshot_date = DateTime::<Utc>::MIN_UTC;
    //
    // for snapshot in snapshots {
    //     if snapshot.date > latest_snapshot_date {
    //         latest_snapshot_date = snapshot.date;
    //     }
    // }
    //
    // return latest_snapshot_date;
}

fn delete_old_snapshots(snapshots: Vec<Snapshot>, max_snapshots: u32) {
    if max_snapshots as usize >= snapshots.len() {
        return;
    }

    let to_delete = snapshots.len() - max_snapshots as usize;

    // if to_delete > 5 {
    //     to_delete = 5;
    // }

    for snapshot_idx in 0..to_delete {
        // out of bounds index should be impossible here
        let folder = &snapshots[snapshot_idx].folder;

        println!("Deleting: {}", folder.display());

        match fs::remove_dir_all(folder) {
            Ok(()) => {}
            Err(e) => {
                log::error!("Could not delete folder: {}: {}", e, folder.display());
            }
        };
    }
}
