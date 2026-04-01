use std::{collections::HashMap, str::FromStr};

use shared::structs::{db::NotificationInsert, metric_type_enum::MetricTypeEnum};
use sqlx::types::chrono;

use crate::{
    AppState, WebSocketType,
    db::{self, Pool},
};

// HashMap<(component name, metric type), (threshold value, actual values)>
type MetricsMap = HashMap<(String, String), (f64, Vec<f64>)>;

pub async fn handle_metrics(state: &AppState, collector_id: i32) -> Result<(), shared::Error> {
    let map = evalueate_metrics(&state.pool, collector_id).await?;
    let map = match map {
        Some(val) => val,
        None => return Ok(()),
    };

    let notif_inserts = create_notifications(&state.pool, collector_id, map).await;
    if notif_inserts.is_empty() {
        return Ok(());
    }

    // send to db, which returns whole notifications with IDs
    let notifications =
        db::insert_collector_notifications(&state.pool, collector_id, notif_inserts).await?;

    // send to broadcast to websocket
    let _ = state
        .tx
        .send((WebSocketType::Notifications(notifications), collector_id));

    Ok(())
}

async fn evalueate_metrics(
    pool: &Pool,
    collector_id: i32,
) -> Result<Option<MetricsMap>, shared::Error> {
    let mut map: MetricsMap = MetricsMap::new();

    let thresholds = crate::db::get_collector_thresholds(pool, collector_id).await?;
    if thresholds.is_empty() {
        return Ok(None);
    }
    // insert key and threshold values to the map
    for t in thresholds {
        map.entry((t.component_name, t.metric_type))
            .or_insert((t.value, vec![]));
    }

    // TODO each metric chould have different value, idk how to fix this rn
    let metrics = crate::db::get_collector_metrics_table(pool, collector_id, Some(5)).await?;
    if metrics.is_empty() {
        return Ok(None);
    }

    // insert actual values to the map
    for m in metrics {
        map.entry((m.component_name, m.metric_type))
            .and_modify(|(_, val)| {
                val.push(m.value);
            });
    }

    Ok(Some(map))
}

async fn create_notifications(
    pool: &Pool,
    collector_id: i32,
    map: MetricsMap,
) -> Vec<NotificationInsert> {
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

    notifications
}
