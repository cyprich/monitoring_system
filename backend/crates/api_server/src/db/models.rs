use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectorDB {
    pub id: i32,
    pub name: String,
    pub system_name: Option<String>,
    pub host_name: Option<String>,
    pub kernel_version: Option<String>,
    pub total_memory_mb: Option<i32>,
    pub cpu_count: Option<i32>,
}
