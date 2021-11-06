///! # PackDb
///! PackDb is a simple key value messagepack store
///! Inspired by [kwik](https://deno.land/x/kwik/)
///! It uses your local storage
///! ## Example
/// ```rs
/// use pack_db::PackDb:
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, PartialEq, Deserialize, Serialize)]
/// struct User {
///     name: String,
///     age: i32
/// }
///
/// let store = PackDb::<User>::new(Some("data".to_owned()));
/// store.set("user1", User {name: "useer1", age: 16});
/// let user = store.get("user1");
///```
use anyhow::Result;
use rmp_serde::Serializer;
use serde::de::DeserializeOwned;
// It is actually being used (?)
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, metadata, read, read_dir, remove_file, write};

pub struct PackDb<T: DeserializeOwned + Serialize> {
    store: String,
    //Yeah idk how to do this in a better way sorry everyone!
    #[allow(dead_code)]
    a: Option<T>,
}
impl<T: DeserializeOwned + Serialize> PackDb<T> {
    //Create a new store
    pub fn new(store: Option<String>) -> Self {
        let loc = store.unwrap_or_else(|| "data".into());
        create_dir_all(&loc).unwrap();
        PackDb::<T> {
            store: loc,
            a: None,
        }
    }
    fn path<K: std::fmt::Display>(&self, key: K) -> String {
        format!("{}/{}.pak", &self.store, key)
    }
    /// Check if a item exists
    pub fn has<K: std::fmt::Display>(&self, key: K) -> bool {
        let exists = metadata(self.path(key));
        exists.is_ok()
    }
    /// Set a item
    pub fn set<K: std::fmt::Display>(&self, key: K, val: T) -> Result<()> {
        let mut buf = vec![];
        val.serialize(&mut Serializer::new(&mut buf))?;
        write(self.path(key), buf)?;
        Ok(())
    }
    /// Get a key
    pub fn get<K: std::fmt::Display>(&self, key: K) -> Result<T> {
        let r = read(self.path(key))?;
        Ok(rmp_serde::from_read_ref(&r)?)
    }

    /// Recieve every object in the store
    pub fn get_all(&self) -> Result<HashMap<String, T>> {
        let mut res = HashMap::new();
        let entries = read_dir(&self.store)?;

        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().replace(".pak", "");
            res.insert(name.clone(), self.get(&name)?);
        }

        Ok(res)
    }
    /// List all keys
    pub fn keys(&self) -> Result<Vec<String>> {
        let mut res = vec![];
        let entries = read_dir(&self.store)?;
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().replace(".pak", "");
            res.push(name);
        }
        Ok(res)
    }
    ///Delete a key - this deletes the file from the file system
    pub fn delete<K: std::fmt::Display>(&self, key: K) -> bool {
        let exists = remove_file(self.path(key));
        exists.is_ok()
    }
}

#[cfg(test)]
mod test {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct Human {
        age: u32,
        name: String,
    }

    use super::*;
    #[test]
    fn test() -> Result<()> {
        let storage = PackDb::<Human>::new(Some("store".to_owned()));

        storage.set(
            "testing",
            Human {
                age: 22,
                name: "this is a test".into(),
            },
        )?;

        let user = storage.get("testing")?;

        assert_eq!(user.name, "this is a test".to_owned());

        let users = storage.get_all()?;
        let _user = users.get("testing").unwrap();

        assert_eq!(storage.has("testing"), true);
        assert_eq!(storage.delete("testing"), true);
        assert_eq!(storage.has("testing"), false);
        Ok(())
    }
}
