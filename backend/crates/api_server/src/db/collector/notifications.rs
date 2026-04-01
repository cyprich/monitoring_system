use shared::structs::{db::NotificationInsert, notifications::Notification};
use sqlx::{Postgres, QueryBuilder};

use crate::db::Pool;

pub async fn insert_collector_notifications(
    pool: &Pool,
    collector_id: i32,
    notifications: Vec<NotificationInsert>,
) -> Result<Vec<Notification>, shared::Error> {
    if notifications.is_empty() {
        return Ok(vec![]);
    }

    let mut builder: QueryBuilder<Postgres> =
        QueryBuilder::new("insert into notifications (collector_id, description, timestamp) ");

    builder.push_values(&notifications, |mut b, n| {
        b.push_bind(collector_id)
            .push_bind(&n.description)
            .push_bind(n.timestamp);
    });

    builder.push(" returning id");

    let ids = builder.build_query_scalar::<i32>().fetch_all(pool).await?;

    let result = notifications
        .into_iter()
        .zip(ids)
        .map(|(old, id)| Notification::from_notification_insert(old, id))
        .collect();

    Ok(result)
}

pub async fn get_collector_notifications(
    pool: &Pool,
    collector_id: i32,
) -> Result<Vec<Notification>, shared::Error> {
    let result = sqlx::query_as!(
        Notification,
        "select * from notifications where collector_id = $1",
        collector_id
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}

pub async fn remove_collector_notifications(
    pool: &Pool,
    collector_id: i32,
    notification_id: Option<i32>,
) -> Result<(), shared::Error> {
    let mut builder: QueryBuilder<Postgres> =
        QueryBuilder::new("delete from notifications where collector_id = ");
    builder.push_bind(collector_id);

    if let Some(val) = notification_id {
        builder.push(" and id = ");
        builder.push_bind(val);
    }

    builder.build().execute(pool).await?;

    Ok(())
}
