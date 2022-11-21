use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::makepath;

#[cfg(not(test))]
use crate::config::BASEDIR;

/// This struct holds what we get from the configuration file. For now we use the exact same
/// file as the C version.
///
#[derive(Clone, Debug)]
pub struct User {
    /// User name
    pub name: String,
    /// They want a different shell
    pub shell: Option<Shell>,
    /// Who they are allowed to become, incl. groups like %wheel or @staff
    pub who: Option<Vec<Become>>,
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
    /// Groups are prefixed with `%` or `@`
    ///
    fn from(s: &str) -> Self {
        if s.starts_with('@') || s.starts_with('%') {
            Become::Group(s[1..].to_owned())
        } else {
            Become::User(s.trim().to_owned())
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
#[derive(Clone, Debug)]
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
            _ => Shell(s.to_owned()),
        }
    }
}

impl Shell {
    /// Check the validity of a proposed shell path against `/etc/shells`.
    ///
    pub fn valid(s: &str) -> bool {
        // Check in `/etc/shells`
        //
        #[cfg(not(test))]
        let db: PathBuf = makepath!(BASEDIR, "shells");

        #[cfg(test)]
        let db: PathBuf = makepath!("testdata", "shells");

        // If there is no `/etc/shells`, assume all shells are valid.
        //
        match File::open(db) {
            Ok(fh) => {
                let bf = BufReader::new(fh);

                // Read all lines, keeping only those starting with a "/"
                //
                let list = bf.lines().map(|l| l.unwrap());

                list.filter(|l| l.starts_with('/')).any(|l| l == *s)
            }
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("noone", Become::User("noone".to_string()))]
    #[case("%group", Become::Group("group".to_string()))]
    #[case("@another", Become::Group("another".to_string()))]
    fn test_become_from(#[case] s: &str, #[case] b: Become) {
        let s = Become::from(s);
        assert_eq!(b, s)
    }

    #[rstest]
    #[case("/bin/zsh", true)]
    #[case("/bin/tcsh", true)]
    #[case("/dish", false)]
    #[case("/usr/local/bin/fish", true)]
    fn test_shell_valid(#[case] s: &str, #[case] valid: bool) {
        assert_eq!(valid, Shell::valid(s))
    }
}
