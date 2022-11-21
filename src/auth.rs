use std::collections::HashMap;

use crate::subr::{Become, User};

/// Our database of users and their "roles" aka who they are allowed to become
///
#[derive(Debug)]
pub struct Auth(HashMap<String, Vec<User>>);

impl Auth {
    /// Instantiate a new authentication database
    ///
    pub fn new(a: HashMap<String, Vec<User>>) -> Self {
        Auth(a)
    }

    /// Check whether a given user exist
    ///
    pub fn exist(&self, user: &str) -> bool {
        self.0.contains_key(user)
    }

    /// Check whether a given user is authorized to become someone else
    ///
    pub fn authorized(&self, _who: Become) -> bool {
        false
    }
}

