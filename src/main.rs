pub mod auth;
pub mod cli;
pub mod config;
pub mod macros;
pub mod subr;
pub mod version;

//#[cfg(not(unix))]
//compile_error!("Not available outside of the Unix World");

use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::{crate_description, Parser};
use log::debug;
use log::LevelFilter::{Debug, Info};

use crate::cli::Opts;
use crate::config::Config;
use crate::subr::Become;
use crate::version::{CALIFE_NAME, CALIFE_VERSION};

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    // Exit if needed
    //
    if opts.version {
        println!("{} v{}", CALIFE_NAME, CALIFE_VERSION);
        println!("{}", crate_description!());
        println!("Config in {:?}", Config::default_file());
        return Ok(());
    }

    let lvl = if opts.debug {
        Debug
    } else {
        Info
    };

    // Prepare logging.
    //
    stderrlog::new()
        .verbosity(lvl)
        .init()?;

    debug!("Hello, world! {} = {}", CALIFE_NAME, CALIFE_VERSION);

    // Temp for development
    //
    debug!("Loading from {:?}", Config::default_file());
    let users = Config::load(PathBuf::from("testdata/calife.auth"))?;
    debug!("{:?}", &users);

    // getlogin(3) equiv
    //
    let whoami = user::get_user_name()?;
    debug!("I am {}", whoami);

    // Do we exist in the auth db?
    //
    if !users.exist(&whoami) {
        bail!("We do not exist, sorry");
    }

    // Compute who we want to be
    //
    let who = match opts.who {
        Some(who) => {
            Become::from(who.as_str())
        },
        // No argument so we want to become "root"
        _ => {
            Become::Root
        }
    };

    dbg!(&who);
    debug!("We want to be {:?}", who);
    Ok(())
}
