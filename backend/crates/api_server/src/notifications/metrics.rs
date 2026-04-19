use std::{collections::HashMap, str::FromStr};

use shared::{enums::metric_type::MetricType, structs::db::NotificationInsert};

use crate::{
    AppState, WebSocketType,
    db::{self, Pool},
};

// HashMap<(component name, metric type), (threshold value, measured values)>
type NotificationsMap = HashMap<(String, String), (f64, Vec<f64>)>;

pub async fn handle_metrics(state: &AppState, collector_id: i32) -> Result<(), shared::Error> {
    let map = collect_into_map(&state.pool, collector_id).await?;
    let map = match map {
        Some(val) => val,
        None => return Ok(()),
    };

    let notif_inserts = create_notifications(collector_id, map).await;
    if notif_inserts.is_empty() {
        return Ok(());
    }

    // send to db, which returns whole notifications with IDs
    let notifications = db::insert_notifications(&state.pool, collector_id, notif_inserts).await?;

    // send to broadcast to websocket
    let _ = state
        .tx
        .send((WebSocketType::Notifications(notifications), collector_id));

    Ok(())
}

async fn collect_into_map(
    pool: &Pool,
    collector_id: i32,
) -> Result<Option<NotificationsMap>, shared::Error> {
    let mut map: NotificationsMap = NotificationsMap::new();

    let thresholds = crate::db::get_metrics_thresholds(pool, collector_id).await?;
    if thresholds.is_empty() {
        return Ok(None);
    }
    // insert key and threshold values to the map
    for t in thresholds {
        let values = crate::db::get_metrics_by_type_and_component(
            pool,
            collector_id,
            &t.metric_type,
            &t.component_name,
            t.count,
        )
        .await
        .unwrap_or_default()
        .iter()
        .map(|val| val.value)
        .collect::<Vec<f64>>();

        map.entry((t.component_name, t.metric_type))
            .or_insert((t.value, values));
    }

    Ok(Some(map))
}

async fn create_notifications(collector_id: i32, map: NotificationsMap) -> Vec<NotificationInsert> {
    let mut notifications: Vec<NotificationInsert> = vec![];

    for ((component_name, metric_type), (threshold_value, measured_values)) in map {
        let avg = measured_values.iter().sum::<f64>() / measured_values.len() as f64;
        let avg = (avg * 100.0).round() / 100.0; // 2 decimal places

        if avg <= threshold_value {
            continue;
        }

        let metric_type_enum = MetricType::from_str(&metric_type);

        let unit = match &metric_type_enum {
            Ok(val) => val.unit(),
            Err(_) => None,
        }
        .unwrap_or("".to_string());

        let description = format!(
            "Exceeded threshold ({}{}) - Average value: {}{}",
            threshold_value, unit, avg, unit
        );

        let cause = match metric_type_enum {
            Ok(val) => match val {
                MetricType::CpuUsage | MetricType::UsedMemoryMb | MetricType::UsedSwapMb => {
                    val.to_string_pretty().unwrap_or(metric_type.clone())
                }
                MetricType::DriveUsedSpace => {
                    format!("Used Space (GB) on Drive '{}'", component_name)
                }
                MetricType::NetworkDownload => {
                    format!("Download (KB) on Network Interface {}", component_name)
                }
                MetricType::NetworkUpload => {
                    format!("Upload (KB) on Network Interface {}", component_name)
                }
            },
            Err(_) => metric_type.clone(),
        };

        notifications.push(NotificationInsert {
            collector_id,
            time: chrono::Utc::now(),
            cause,
            description: Some(description),
        });
    }

    notifications
}
