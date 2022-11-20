use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use itertools::Itertools;
use log::debug;
use serde::Deserialize;
use crate::auth::Auth;

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

impl Default for User {
    /// Invalid user role by default
    ///
    fn default() -> Self {
        User {
            name: "NONE".to_owned(),
            shell: None,
            who: None,
        }
    }
}

/// Enum for the users/groups the user will have rights on
///
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Become {
    /// Plain su-like role
    Root,
    /// A specific user
    User(String),
    /// A group
    Group(String),
}

impl Default for Become {
    /// Most basic role
    ///
    fn default() -> Self {
        Become::Root
    }
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

impl Become {
    /// Generate a list of roles from a vector of entries from the file
    ///
    pub fn from_vec(v: Vec<&str>) -> Vec<Self> {
        v.iter().map(|&e| Become::from(e)).collect()
    }
}

/// Holds the path for a shell
///
#[derive(Clone, Deserialize, Debug)]
pub struct Shell(String);

impl Default for Shell {
    /// Reasonable default
    ///
    fn default() -> Self {
        Shell("/bin/zsh".to_string())
    }
}

impl From<&str> for Shell {
    fn from(s: &str) -> Self {
        match s {
            "" => Shell::default(),
            _ => Shell(s.to_owned())
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

/// Empty struct used as "namespace"
///
pub struct Config;

impl Config {
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

#[cfg(test)]
mod tests {
    use anyhow::bail;
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("noone", Become::User("noone".to_string()))]
    #[case("%group", Become::Group("group".to_string()))]
    #[case("@another", Become::Group("another".to_string()))]
    fn test_become_from(#[case] s: &str, #[case]b: Become) {
        let s = Become::from(s);
        assert_eq!(b, s)
    }
}