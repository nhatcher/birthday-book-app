use napi::{self, bindgen_prelude::*};

#[macro_use]
extern crate napi_derive;

use book::{Book, Friend};

#[napi]
pub struct NodeBook {
    book: Book,
}

#[napi(object)]
pub struct NodeFriend {
    pub name: String,
    pub email: String,
    pub last_name: String,
    pub birthday: Option<String>,
}

#[napi]
impl NodeBook {
    #[napi(constructor)]
    pub fn new(owner: String) -> Self {
        let book = Book::new(&owner);
        NodeBook { book }
    }

    #[napi(factory)]
    #[napi(js_name=fromJSON)]
    pub fn from_json_str(json_str: String) -> Result<NodeBook> {
        match Book::from_json_str(&json_str) {
            Ok(book) => Ok(NodeBook { book }),
            Err(s) => Err(Error::new(Status::Unknown, s)),
        }
    }

    #[napi(js_name=toJSON)]
    pub fn to_json_string(&self) -> Result<String> {
        match self.book.to_json_string() {
            Ok(s) => Ok(s),
            Err(s) => Err(Error::new(Status::Unknown, s)),
        }
    }

    #[napi(js_name=getFriendCountWithLastName)]
    pub fn get_friend_count_with_last_name(&self, name: String) -> i32 {
        self.book.friends_with_last_name(&name).len() as i32
    }

    #[napi(js_name=getFirstFriendWithLastName)]
    pub fn get_first_friend_with_last_name(&self, last_name: String) -> Result<NodeFriend> {
        match &self.book.friends_with_last_name(&last_name).get(0) {
            Some(friend) => Ok(NodeFriend {
                name: friend.name.clone(),
                last_name: friend.last_name.clone(),
                email: friend.email.clone(),
                birthday: friend.birthday.clone(),
            }),
            None => Err(Error::new(Status::Unknown, "No friends")),
        }
    }

    #[napi(js_name=addFriend)]
    pub fn add_friend(&mut self, friend: NodeFriend) -> Result<()> {
        let new_friend = Friend {
            name: friend.name,
            last_name: friend.last_name,
            email: friend.email,
            birthday: friend.birthday,
        };
        match self.book.add_friend(new_friend) {
            Ok(_) => Ok(()),
            Err(s) => Err(Error::new(Status::Unknown, s)),
        }
    }

    #[napi(js_name=writeToFile)]
    pub fn write_to_file(&self, file_path: String) -> Result<()> {
        match self.book.write_to_file(&file_path) {
            Ok(_) => Ok(()),
            Err(s) => Err(Error::new(Status::Unknown, s)),
        }
    }

    #[napi(factory)]
    #[napi(js_name=readFromFile)]
    pub fn read_from_file(file_path: String) -> Result<NodeBook> {
        match Book::read_from_file(&file_path) {
            Ok(book) => Ok(NodeBook { book }),
            Err(s) => Err(Error::new(Status::Unknown, s)),
        }
    }

    #[napi(js_name=getOwner)]
    pub fn get_owner(&self) -> String {
        self.book.owner()
    }
}
