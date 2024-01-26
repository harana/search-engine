use rusqlite::{Error, Result};

pub fn decode(str: String) -> Result<String> {
    match urlencoding::decode(str.as_str()) {
        Ok(str) => Ok(str.to_string()),
        Err(e) => Err(Error::Utf8Error(e.utf8_error()))
    }
}

pub fn encode(str: String) -> String {
    urlencoding::encode(str.as_str()).to_string()
}