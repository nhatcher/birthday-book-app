use std::fs;
use std::time::SystemTime;
use uuid::Uuid;

use crate::Book;
/// Returns The Current Epoch Unix Timestamp
/// Number of seconds January 1, 1970 00:00:00 UTC.
pub(crate) fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!")
        .as_secs()
}

pub(crate) fn uuid4() -> String {
    Uuid::new_v4().to_string()
}

impl Book {
    /// Reads the Book from disk
    pub fn read_from_file(file_path: &str) -> Result<Book, String> {
        match fs::read_to_string(file_path) {
            Ok(s) => Book::from_json_str(&s),
            Err(_) => Err("Error reading file".to_string()),
        }
    }

    /// Saves to Book to disk
    pub fn write_to_file(&self, file_path: &str) -> Result<(), String> {
        let s = match self.to_json_string() {
            Ok(s) => s,
            Err(s) => return Err(s),
        };
        match fs::write(file_path, s) {
            Ok(()) => Ok(()),
            Err(_) => Err("Error writing to file".to_string()),
        }
    }
}
