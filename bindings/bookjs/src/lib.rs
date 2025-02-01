use serde::{Deserialize, Serialize};
use tsify::Tsify;

use book::{Book, Friend};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct WasmBook {
    book: Book,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WasmFriend {
    pub name: String,
    pub email: String,
    pub last_name: String,
    pub birthday: Option<String>,
}

#[wasm_bindgen]
impl WasmBook {
    #[wasm_bindgen(constructor)]
    pub fn new(owner: &str) -> WasmBook {
        let book = Book::new(owner);
        WasmBook { book }
    }

    #[wasm_bindgen(js_name=fromJSON)]
    pub fn from_json_str(json_str: &str) -> Result<WasmBook, String> {
        match Book::from_json_str(json_str) {
            Ok(book) => Ok(WasmBook { book }),
            Err(s) => Err(s),
        }
    }

    #[wasm_bindgen(js_name=toJSON)]
    pub fn to_json_string(&self) -> Result<String, String> {
        self.book.to_json_string()
    }

    #[wasm_bindgen(js_name=getFriendCountWithLastName)]
    pub fn get_friend_count_with_last_name(&self, name: &str) -> usize {
        self.book.friends_with_last_name(name).len()
    }

    #[wasm_bindgen(js_name=getFirstFriendWithLastName)]
    pub fn get_first_friend_with_last_name(&self, last_name: &str) -> Result<WasmFriend, String> {
        match &self.book.friends_with_last_name(last_name).get(0) {
            Some(friend) => Ok(WasmFriend {
                name: friend.name.clone(),
                last_name: friend.last_name.clone(),
                email: friend.email.clone(),
                birthday: friend.birthday.clone(),
            }),
            None => Err("No friends".to_string()),
        }
    }

    #[wasm_bindgen(js_name=addFriend)]
    pub fn add_friend(&mut self, friend: WasmFriend) -> Result<(), String> {
        let new_friend = Friend {
            name: friend.name,
            last_name: friend.last_name,
            email: friend.email,
            birthday: friend.birthday,
        };
        match self.book.add_friend(new_friend) {
            Ok(_) => Ok(()),
            Err(s) => Err(s),
        }
    }
}
