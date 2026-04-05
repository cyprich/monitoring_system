use shared::structs::{db::NotificationInsert, notifications::Notification};

use crate::db::{Builder, Pool};

pub async fn insert_notifications(
    pool: &Pool,
    collector_id: i32,
    notifications: Vec<NotificationInsert>,
) -> Result<Vec<Notification>, shared::Error> {
    if notifications.is_empty() {
        return Ok(vec![]);
    }

    let mut builder =
        Builder::new("insert into notifications (collector_id, cause, description, time) ");

    builder.push_values(&notifications, |mut b, n| {
        b.push_bind(collector_id)
            .push_bind(&n.cause)
            .push_bind(&n.description)
            .push_bind(n.time);
    });

    builder.push(" returning *");

    let result = builder
        .build_query_as::<Notification>()
        .fetch_all(pool)
        .await?;

    Ok(result)
}

pub async fn get_notifications(
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

pub async fn delete_notification(
    pool: &Pool,
    collector_id: i32,
    notification_id: Option<i32>,
) -> Result<(), shared::Error> {
    let mut builder = Builder::new("delete from notifications where collector_id = ");
    builder.push_bind(collector_id);

    if let Some(val) = notification_id {
        builder.push(" and id = ");
        builder.push_bind(val);
    }

    builder.build().execute(pool).await?;

    Ok(())
}
