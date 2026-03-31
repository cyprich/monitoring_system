use std::{collections::HashMap, str::FromStr};

use shared::structs::{db::NotificationInsert, metric_type_enum::MetricTypeEnum};
use sqlx::types::chrono;

use crate::db::{self, Pool};

// TODO it's making notifications too often, maybe some cooldown would be nice

// TODO split into two functions - evaluate and insert
pub async fn handle_metrics(pool: &Pool, collector_id: i32) -> Result<(), shared::Error> {
    // HashMap<(component name, metric type), (threshold value, actual values)>
    let mut map: HashMap<(String, String), (f64, Vec<f64>)> = HashMap::new();

    let thresholds = crate::db::get_collector_thresholds(pool, collector_id).await?;
    if thresholds.is_empty() {
        return Ok(());
    }
    // insert key and threshold values to the map
    for t in thresholds {
        map.entry((t.component_name, t.metric_type))
            .or_insert((t.value, vec![]));
    }

    // TODO each metric chould have different value, idk how to fix this rn
    let metrics = crate::db::get_collector_metrics_table(pool, collector_id, Some(5)).await?;
    if metrics.is_empty() {
        return Ok(());
    }

    // insert actual values to the map
    for m in metrics {
        map.entry((m.component_name, m.metric_type))
            .and_modify(|(_, val)| {
                val.push(m.value);
            });
    }

    // creating notifications
    let mut notifications: Vec<NotificationInsert> = vec![];
    let collector_name = db::get_collector_name(pool, collector_id)
        .await
        .unwrap_or(format!("#{collector_id}"));

    'outer: for ((component_name, metric_type), (threshold_value, actual_values)) in map {
        for val in &actual_values {
            if val < &threshold_value {
                continue 'outer;
            }
        }

        let mut cause_name = match MetricTypeEnum::from_str(&metric_type) {
            Ok(val) => val.to_string_pretty().unwrap_or(metric_type),
            Err(_) => metric_type,
        };

        if !component_name.is_empty() {
            cause_name.push_str(&component_name);
        }

        let values = actual_values
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let description = format!(
            "Value of '{}' of collector '{}' exceeded threshold of '{}'! Measured values: {}",
            cause_name, collector_name, threshold_value, values
        );

        notifications.push(NotificationInsert {
            collector_id,
            description,
            timestamp: chrono::Local::now().naive_local(),
            viewed: false,
        });
    }

    db::insert_collector_notifications(pool, collector_id, notifications).await?;

    Ok(())
}

// pub async fn handle_endpoints(
//     pool: &Pool,
//     metrics: &Metrics,
//     collector_id: i32,
// ) -> Result<Vec<NotificationInsert>, shared::Error> {
//     let result = crate::db::get_collector_thresholds(pool, collector_id).await?;
//
//     Ok(vec![])
// }
