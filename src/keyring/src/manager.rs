use keyring::{Entry, Error};

pub struct KeyringManager {}

impl KeyringManager {

    fn get_secret(name: String) -> Result<Vec<u8>, Error> {
        let entry = Entry::new(name.as_str(), whoami::username().as_str());
        entry.and_then(|e| e.get_secret())
    }

    fn set_secret(name: String, secret: Vec<u8>) -> Result<(), Error> {
        let entry = Entry::new(name.as_str(), whoami::username().as_str());
        entry.and_then(|e| e.set_secret(secret.as_slice()))
    }

    fn delete_secret(name: String) -> Result<(), Error> {
        let entry = Entry::new(name.as_str(), whoami::username().as_str());
        entry.and_then(|e| e.delete_credential())
    }

}