use std::collections::BTreeMap;
use std::collections::HashMap;

struct TimeMap {
    key_timestamp_map: HashMap<String, BTreeMap<i32, String>>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            key_timestamp_map: Default::default(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.key_timestamp_map
            .entry(key)
            .or_default()
            .insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.key_timestamp_map
            .get(&key)
            .and_then(|lookup| lookup.range(..=timestamp).next_back())
            .map(|(_, val)| val.to_owned())
            .unwrap_or_default()
    }
}
