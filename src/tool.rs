use crate::{
    fs::FileCrawler,
    settings::{self, Config},
    types,
};

pub fn backup(profile: &String) -> types::BackupResult<()> {
    let config = settings::Backup::load(profile)?;
    println!("Loaded: {:#?}", config);

    let crawler = FileCrawler::from(&config.source_path);

    for file in crawler {
        println!("path: {}", file.display());
    }

    Ok(())
}
