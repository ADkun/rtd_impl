mod inc_counter;

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub use inc_counter::IncCounter;

pub fn get_current_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("系统时间应当在1970年之后")
        .as_secs()
}

pub fn path_exists(path: &str) -> bool {
    let path = Path::new(path);
    path.exists()
}
