use shared::structs::{db::NotificationInsert, ports::Port};

use crate::{AppState, WebSocketType, db};

pub async fn handle_ports(
    state: &AppState,
    collector_id: i32,
    ports: Vec<Port>,
    is_handling_opened: bool,
) -> Result<(), shared::Error> {
    let val = db::get_collector_ports_notifications_settings(&state.pool, collector_id).await?;
    match is_handling_opened {
        true => {
            if !val.show_for_opened {
                return Ok(());
            }
        }
        false => {
            if !val.show_for_closed {
                return Ok(());
            }
        }
    }

    let cause_value = match is_handling_opened {
        true => "Opened",
        false => "Closed",
    };

    let notifications = ports
        .iter()
        .map(|p| NotificationInsert {
            collector_id,
            cause: format!("Port {}", cause_value),
            description: Some(format!(
                "{} port {} was opened on address '{}'",
                p.protocol, p.port, p.address
            )),
            time: p.last_update,
        })
        .collect::<Vec<NotificationInsert>>();

    let notifications = db::insert_notifications(&state.pool, collector_id, notifications).await?;

    let _ = state
        .tx
        .send((WebSocketType::Notifications(notifications), collector_id));

    Ok(())
}
