use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::structs::db::NotificationInsert;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: i32,
    pub collector_id: i32,
    pub description: String,
    pub timestamp: NaiveDateTime,
    pub viewed: bool,
}

impl Notification {
    pub fn from_notification_insert(value: NotificationInsert, id: i32) -> Self {
        Self {
            id,
            collector_id: value.collector_id,
            description: value.description,
            timestamp: value.timestamp,
            viewed: value.viewed,
        }
    }
}
