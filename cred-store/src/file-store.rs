use super::traits::CredStore;
use dirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::path::Path;

const CREDENTIALS_FILE: &str = ".mg/credentials.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    data: HashMap<String, String>,
    file_name: String,
}

impl Credentials {
    pub fn new() -> Self {
        Credentials {
            data: HashMap::new(),
            file_name: CREDENTIALS_FILE.to_string(),
        }
    }

    pub fn set_file_name(mut self, file_name: String) -> Self {
        self.file_name = file_name;
        self
    }

    pub fn build(&self) -> Self {
        Credentials {
            data: self.data.clone(),
            file_name: self.file_name.clone(),
        }
    }
}

impl Default for Credentials {
    fn default() -> Self {
        Credentials::new()
    }
}

impl CredStore for Credentials {
    fn add(&mut self, key: String, value: String) -> &mut Self {
        self.data.insert(key, value);
        self
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn clear(&mut self) -> &mut Self {
        self.data.clear();
        self
    }

    fn keys_present(&self, keys: &[String]) -> bool {
        keys.iter().all(|key| self.data.contains_key(key))
    }

    fn load(&self) -> Result<Self, Error> {
        let store_path = match dirs::home_dir() {
            Some(path) => path.join(&self.file_name),
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    "Home directory not found",
                ))
            }
        };
        if Path::new(&store_path).exists() {
            let contents = fs::read_to_string(&store_path)?;
            let data: HashMap<String, String> = serde_json::from_str(&contents)?;
            Ok(Credentials {
                data,
                file_name: self.file_name.clone(),
            })
        } else {
            Ok(Credentials {
                data: HashMap::new(),
                file_name: self.file_name.clone(),
            })
        }
    }

    fn save(&self) -> Result<(), Error> {
        let store_path = match dirs::home_dir() {
            Some(path) => path.join(&self.file_name),
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    "Home directory not found",
                ))
            }
        };
        if let Some(parent) = store_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(&self.data)?;
        fs::write(store_path, contents)?;
        Ok(())
    }

    fn delete(&self) -> Result<(), Error> {
        let store_path = match dirs::home_dir() {
            Some(path) => path.join(&self.file_name),
            None => {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    "Home directory not found",
                ))
            }
        };
        if Path::new(&store_path).exists() {
            fs::remove_file(store_path)?;
        }
        Ok(())
    }
}
