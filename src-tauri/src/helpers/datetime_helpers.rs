use std::time::{SystemTime, UNIX_EPOCH};

// Helper para obtener el timestamp actual
pub fn timestamp_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}