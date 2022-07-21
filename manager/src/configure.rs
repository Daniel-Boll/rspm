// Const var of cache dir
use std::fs;

pub(crate) use crate::file::fetch_cache;

/// Stores in the cache located at $HOME/.cache/spm/config.json the file location
pub fn define_file_location(location: String) {
    let mut cache = fetch_cache();
    let cache_file = format!("{}/.cache/spm/config.json", std::env::var("HOME").unwrap());
    cache.file_location = location;
    let json = serde_json::to_string(&cache).unwrap();
    fs::write(cache_file, json).unwrap();
}
