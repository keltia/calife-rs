use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use itertools::Itertools;
use log::debug;

use crate::auth::Auth;
use crate::makepath;
use crate::subr::{Become, Shell, User};

pub const BASEDIR: &str = "/etc";
pub const CONFIG_FILE: &str = "calife.auth";

/// Empty struct used as "namespace"
///
pub struct Config;

impl Config {
    /// Returns the path of the default config file
    ///
    pub fn default_file() -> PathBuf {
        let def: PathBuf = makepath!(BASEDIR, CONFIG_FILE);
        def
    }

    /// Basic reader for the original configuration file
    ///
    pub fn load(fname: PathBuf) -> Result<Auth> {
        let lines = fs::read_to_string(fname)?;
        let content = lines.lines();
        let users = content
            .map(|line| {
                let fields: Vec<_> = line.split(":").collect();
                let n = fields[0];

                // Better way to do this maybe?
                //
                match fields.len() {
                    1 => User {
                        name: n.to_owned(),
                        shell: Some(Shell::default()),
                        who: Some(vec![Become::Root]),
                    },
                    2 => User {
                        name: n.to_owned(),
                        shell: Some(Shell::from(fields[1])),
                        who: Some(vec![Become::Root]),
                    },
                    3 => {
                        let users: Vec<&str> = fields[2].split(",").collect();
                        User {
                            name: n.to_owned(),
                            shell: Some(Shell::from(fields[1])),
                            who: Some(Become::from_vec(users)),
                        }
                    }
                    _ => User {
                        name: "INVALID".to_owned(),
                        shell: None,
                        who: None,
                    },
                }
            })
            // Filter out invalid entries
            //
            .filter(|u| u.name != "INVALID")
            .map(|u| (u.name.to_owned(), u))
            // Create a HashMap of all entries
            //
            .into_group_map();

        Ok(Auth::new(users))
    }
}

