//! This module provides the feature of persistent data store via file system.

use crate::cache::{CacheInfo, MAX_DIGESTS};
use crate::dirs::PROJECT_DIRS;
use crate::recent_files::SortedRecentFiles;
use crate::stdio_server::InputHistory;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Linux: ~/.local/share/vimclap/cache.json
const CACHE_FILENAME: &str = "cache.json";

static CACHE_METADATA_PATH: Lazy<Option<PathBuf>> =
    Lazy::new(|| generate_data_file_path(CACHE_FILENAME).ok());

pub static CACHE_INFO_IN_MEMORY: Lazy<Arc<Mutex<CacheInfo>>> = Lazy::new(|| {
    let mut maybe_persistent = load_json::<CacheInfo, _>(CACHE_METADATA_PATH.as_deref())
        .unwrap_or_else(|| CacheInfo::with_capacity(MAX_DIGESTS));
    maybe_persistent.remove_invalid_and_old_entries();
    Arc::new(Mutex::new(maybe_persistent))
});

/// Linux: ~/.local/share/vimclap/recent_files.json
const RECENT_FILES_FILENAME: &str = "recent_files.json";

static RECENT_FILES_JSON_PATH: Lazy<Option<PathBuf>> =
    Lazy::new(|| generate_data_file_path(RECENT_FILES_FILENAME).ok());

pub static RECENT_FILES_IN_MEMORY: 