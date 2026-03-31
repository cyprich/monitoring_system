use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub id: i32,
    pub collector_id: i32,
    pub description: String,
    pub timestamp: NaiveDateTime,
    pub viewed: bool,
}
