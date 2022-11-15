mod config;
mod version;
mod macros;

#[cfg(not(unix))]
compile_error!("Not available outside of the Unix World");

use anyhow::Result;
use csv::ReaderBuilder;

use crate::config::{CONFIG_FILE, User};
use crate::version::{CALIFE_NAME, CALIFE_VERSION};

fn main() -> Result<()> {
    println!("Hello, world! {} = {}", CALIFE_NAME, CALIFE_VERSION);

    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .delimiter(b':')
        .from_path(CONFIG_FILE)
        .unwrap_or_else(Err(anyhow!("Unknown file {}", CONFIG_FILE)))?;

    let _users: Vec<_> = rdr
        .records()
        .map(|user| {
            let user = user.unwrap();

            let user: User = user.deserialize(None).unwrap();
            user
        }).collect();

    Ok(())
}
