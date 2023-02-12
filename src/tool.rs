use crate::{types, settings::{self, Config}};

pub fn backup(profile: &String) -> types::BackupResult<()> {
    let config = settings::Backup::load(profile)?;
    println!("Loaded: {:#?}", config);
    
    Ok(())
}