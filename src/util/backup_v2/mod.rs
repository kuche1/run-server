use chrono::{DateTime, Duration, Utc};
use clonetree::{Options, clone_tree};
use std::fs;
use std::path::Path;

pub fn main(
    game_data_folder: &Path,
    backups_folder: &Path,
    cooldown_days: f64,
) -> anyhow::Result<()> {
    let now = Utc::now();

    if is_on_cooldown(backups_folder, cooldown_days, now)? {
        return Ok(());
    }

    let destination = backups_folder.join(now.to_rfc3339());
    clone_tree(game_data_folder, destination, &Options::new())?;

    Ok(())
}

/// If the cooldown is 1 day and we call this exactly after 1 day, the backup is going to be created
fn is_on_cooldown(
    backups_folder: &Path,
    cooldown_days: f64,
    now: DateTime<Utc>,
) -> anyhow::Result<bool> {
    let Some(latest) = latest_backup_date(backups_folder)? else {
        return Ok(false);
    };

    let cooldown = Duration::seconds((cooldown_days * 24.0 * 60.0 * 60.0) as i64);

    Ok(now < latest + cooldown)
}

fn latest_backup_date(backups_folder: &Path) -> anyhow::Result<Option<DateTime<Utc>>> {
    if !backups_folder.exists() {
        return Ok(None);
    }

    let mut latest = None;

    for entry in fs::read_dir(backups_folder)? {
        let name = entry?.file_name();
        let name = name.to_string_lossy();

        let Ok(date) = name.parse::<DateTime<Utc>>() else {
            anyhow::bail!("Not a date: {name}");
        };

        if latest.is_none_or(|l| date > l) {
            latest = Some(date);
        }
    }

    Ok(latest)
}
