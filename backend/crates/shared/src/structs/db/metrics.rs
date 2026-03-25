use chrono::NaiveDateTime;

use crate::structs::db::metric_type::MetricType;

#[derive(Debug, sqlx::FromRow)]
pub struct MetricsDB {
    pub timestamp: NaiveDateTime,
    pub value: f64,
    pub metric_type: MetricType,
    pub collector_id: i32,
    pub component_name: String,
}
