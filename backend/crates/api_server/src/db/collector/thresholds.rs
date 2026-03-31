use shared::structs::db::ThresholdsTable;

use crate::db::Pool;

pub async fn get_collector_thresholds(
    pool: &Pool,
    collector_id: i32,
) -> Result<Vec<ThresholdsTable>, shared::Error> {
    let result = sqlx::query_as!(
        ThresholdsTable,
        "select * from thresholds where collector_id = $1",
        collector_id
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}
