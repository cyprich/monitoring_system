use shared::structs::db::NotificationInsert;
use sqlx::{Postgres, QueryBuilder};

use crate::db::Pool;

pub async fn insert_collector_notifications(
    pool: &Pool,
    collector_id: i32,
    notifications: Vec<NotificationInsert>,
) -> Result<(), shared::Error> {
    if notifications.is_empty() {
        return Ok(());
    }

    let mut builder: QueryBuilder<Postgres> =
        QueryBuilder::new("insert into notifications (collector_id, description, timestamp) ");

    builder.push_values(notifications, |mut b, n| {
        b.push_bind(collector_id)
            .push_bind(n.description)
            .push_bind(n.timestamp);
    });

    builder.build().execute(pool).await?;

    Ok(())
}
