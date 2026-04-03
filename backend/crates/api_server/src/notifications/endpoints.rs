use std::collections::HashMap;

use shared::structs::db::NotificationInsert;

use crate::{
    AppState, WebSocketType,
    db::{Pool, get_endpoints_by_id, get_endpoints_thresholds, insert_notifications},
};

// HashMap<endpoint id, (threshold value, measured values)>
type EndpointsMap = HashMap<i32, (i32, Vec<bool>)>;

pub async fn handle_endpoints(state: &AppState, collector_id: i32) -> Result<(), shared::Error> {
    let map = evaluate(&state.pool, collector_id).await?;
    let map = match map {
        Some(val) => val,
        None => return Ok(()),
    };

    let notif_inserts = create_notifications(&state.pool, collector_id, map).await?;
    if notif_inserts.is_empty() {
        return Ok(());
    }

    // send to db, which returns whole notifications with IDs
    let notifications = insert_notifications(&state.pool, collector_id, notif_inserts).await?;

    // send to broadcast to websocket
    let _ = state
        .tx
        .send((WebSocketType::Notifications(notifications), collector_id));

    Ok(())
}

async fn evaluate(pool: &Pool, collector_id: i32) -> Result<Option<EndpointsMap>, shared::Error> {
    let mut map: EndpointsMap = EndpointsMap::new();

    // TODO does this needs to be the full variant?
    // let thresholds = crate::db::get_collector_endpoints_thresholds_join(pool, collector_id).await?;
    // if thresholds.is_empty() {
    //     return Ok(None);
    // }

    let thresholds = get_endpoints_thresholds(pool, collector_id).await?;

    // insert key and threshold values to the map
    for t in thresholds {
        map.entry(t.endpoint_id).or_insert((t.value, vec![]));
    }

    // TODO each metric chould have different value (limit), idk how to fix this rn - needs another
    // field in db
    let endpoints_results = crate::db::get_endpoints_results(pool, collector_id, Some(5)).await?;

    if endpoints_results.is_empty() {
        return Ok(None);
    }

    // insert actual values to the map
    for e in endpoints_results {
        map.entry(e.endpoint_id).and_modify(|(_, val)| {
            val.push(e.result);
        });
    }

    Ok(Some(map))
}

async fn create_notifications(
    pool: &Pool,
    collector_id: i32,
    map: EndpointsMap,
) -> Result<Vec<NotificationInsert>, shared::Error> {
    let mut notifications: Vec<NotificationInsert> = vec![];
    let endpoints_by_id = get_endpoints_by_id(pool, Some(collector_id)).await?;

    'outer: for (endpoint_id, (threshold_value, measured_values)) in map {
        for val in &measured_values {
            if *val {
                continue 'outer;
            }
        }

        let url = match endpoints_by_id.get(&endpoint_id) {
            Some(val) => &val.url,
            None => &format!("ID: {}", endpoint_id),
        };

        let pretty_component_name = format!("Too many unsucesfull requests for endpoint '{}'", url);

        notifications.push(NotificationInsert {
            collector_id,
            timestamp: sqlx::types::chrono::Local::now().naive_local(),
            metric_type: "endpoint".to_string(),
            component_name: pretty_component_name,
            threshold_value: threshold_value as f64,
            measured_values: vec![],
        });
    }

    Ok(notifications)
}
