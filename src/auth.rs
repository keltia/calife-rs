use std::collections::HashMap;

use crate::config::{Become, User};

/// Our database of users and their "roles" aka who they are allowed to become
///
#[derive(Debug)]
pub struct Auth(HashMap<String, Vec<User>>);

impl Auth {
    pub fn new(a: HashMap<String, Vec<User>>) -> Self {
        Auth(a)
    }

    pub fn exist(&self, user: &str) -> bool {
        self.0.contains_key(user)
    }

    pub fn authorized(&self, who: Become) -> bool {
        false
    }
}

