mod structs;
pub use crate::structs::Collector;

const UNKNOWN: &str = "<<unknown>>";

pub fn get_supported_systems() -> Vec<String> {
    vec![
        "Android".to_string(),
        "FreeBSD".to_string(),
        "NetBSD".to_string(),
        "iOS".to_string(),
        "Linux".to_string(),
        "macOS".to_string(),
        "Raspberry Pi".to_string(),
        "Windows".to_string(),
    ]
}

fn unwrap_or_unknown(val: Option<String>, variable_name: &str) -> String {
    match val {
        Some(val) => val,
        None => {
            log::warn!("Couldn't get {}", variable_name);
            UNKNOWN.to_string()
        }
    }
}
