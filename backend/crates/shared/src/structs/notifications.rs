use serde::{Deserialize, Serialize};

use crate::structs::db::NotificationInsert;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct Notification {
    pub id: i32,
    pub collector_id: i32,
    pub cause: String,
    pub description: Option<String>,
    pub time: chrono::DateTime<chrono::Utc>,
}

impl Notification {
    pub fn from_notification_insert(value: NotificationInsert, id: i32) -> Self {
        Self {
            id,
            collector_id: value.collector_id,
            cause: value.cause,
            description: value.description,
            time: value.time,
        }
    }
}
