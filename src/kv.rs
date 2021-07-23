use std::collections::HashMap;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Default, Debug)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`.
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Option<String> {
        let v = self.map.get(&key);
        v.cloned()
    }

    ///Example of remove
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut kv_store = KvStore::new();
    /// kv_store.set("name".to_string(), "ratul".to_string());
    /// assert_eq!(kv_store.get("name".to_string()), Some("ratul".to_owned()));
    /// kv_store.remove("name".to_owned());
    /// assert_eq!(kv_store.get("name".to_string()), None);
    /// ```
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
