use std::path::PathBuf;

use csv::Reader;
use log::debug;
use serde::Deserialize;

use crate::makepath;

pub const BASEDIR: &str = "/etc";
pub const CONFIG_FILE: &str = "calife.auth";

#[derive(Debug, Deserialize)]
pub struct User {
    /// Who is allowed
    pub name: String,
    /// They want a different shell
    pub shell: Option<String>,
    /// Who they are allowed to become, incl. groups like %wheel or @staff
    pub users: Option<Vec<Become>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Become {
    User(String),
    Group(String),
}

impl From<&str> for Become {
    fn from(s: &str) -> Self {
        if s.starts_with("@") || s.starts_with("%") {
            Become::Group(s[1..].to_owned())
        } else {
            Become::User(s[1..].to_owned())
        }
    }
}

/// Returns the path of the default config file
///
pub fn default_file() -> PathBuf {
    let def: PathBuf = makepath!(BASEDIR, CONFIG_FILE);
    debug!("Default file: {:?}", def);
    def
}

