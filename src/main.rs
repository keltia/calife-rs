pub mod auth;
pub mod cli;
pub mod config;
pub mod macros;
pub mod subr;
pub mod version;

#[cfg(not(unix))]
//compile_error!("Not available outside of the Unix World");
use std::path::PathBuf;

use anyhow::Result;
use clap::{crate_description, Parser};
use log::debug;
use log::LevelFilter::{Debug, Info};

use crate::config::{User, CONFIG_FILE, Config, BASEDIR};
use crate::version::{CALIFE_NAME, CALIFE_VERSION};

fn main() -> Result<()> {
    println!("Hello, world! {} = {}", CALIFE_NAME, CALIFE_VERSION);
    let def: PathBuf = makepath!(BASEDIR, CONFIG_FILE);
    println!("Default config in {:?}", def);

    // Temp for devlopment
    //
    let users = Config::load(PathBuf::from("testdata/calife.auth"))?;
    dbg!(&users);

    Ok(())
}
