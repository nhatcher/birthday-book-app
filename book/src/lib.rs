mod target;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use target::{get_timestamp, uuid4};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Friend {
    pub name: String,
    pub email: String,
    pub last_name: String,
    pub birthday: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Book {
    owner: String,
    last_edited: u64,
    friends: HashMap<String, Friend>,
}

impl Book {
    /// Creates a new empty Book
    pub fn new(owner: &str) -> Book {
        Book {
            owner: owner.to_owned(),
            last_edited: get_timestamp(),
            friends: HashMap::new(),
        }
    }

    /// Creates a Book object from it's JSON representation
    pub fn from_json_str(json_str: &str) -> Result<Book, String> {
        match serde_json::from_str(json_str) {
            Ok(book) => Ok(book),
            Err(_) => Err("Invalid book!".to_string()),
        }
    }

    /// Returns a JSON representation of the Book object
    pub fn to_json_string(&self) -> Result<String, String> {
        match serde_json::to_string(&self) {
            Ok(s) => Ok(s),
            Err(_) => Err("Unknown error".to_string()),
        }
    }

    /// Gives me the list of my friends with a particular last name
    pub fn friends_with_last_name(&self, last_name: &str) -> Vec<Friend> {
        let mut friends = Vec::new();
        for friend in self.friends.values() {
            if friend.last_name == last_name {
                friends.push(friend.clone())
            }
        }
        friends
    }

    /// Adds a new friend, fails if you have the friend in the book
    pub fn add_friend(&mut self, new_friend: Friend) -> Result<(), String> {
        for friend in self.friends.values() {
            if friend == &new_friend {
                return Err("You already have that friend".to_owned());
            }
        }
        let key = uuid4();
        self.friends.insert(key, new_friend);
        self.last_edited = get_timestamp();
        Ok(())
    }

    // Returns the book's owner
    pub fn owner(&self) -> String {
        self.owner.to_string()
    }
}
