use crate::models::game::simple_property::UserSimpleProperty;
use anyhow::Result;
use sqlx::SqlitePool;

/// Get all simple properties for a user
pub async fn get_simple_properties(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<UserSimpleProperty>> {
    let properties = sqlx::query_as::<_, UserSimpleProperty>(
        "SELECT user_id, property_id, property_value FROM user_simple_properties WHERE user_id = ? ORDER BY property_id"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(properties)
}

/// Set a simple property (upsert)
pub async fn set_simple_property(
    pool: &SqlitePool,
    user_id: i64,
    property_id: i32,
    property_value: String,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO user_simple_properties (user_id, property_id, property_value)
        VALUES (?, ?, ?)
        ON CONFLICT(user_id, property_id) DO UPDATE SET property_value = excluded.property_value
        "#,
    )
    .bind(user_id)
    .bind(property_id)
    .bind(property_value)
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete a simple property
pub async fn delete_simple_property(
    pool: &SqlitePool,
    user_id: i64,
    property_id: i32,
) -> Result<()> {
    sqlx::query("DELETE FROM user_simple_properties WHERE user_id = ? AND property_id = ?")
        .bind(user_id)
        .bind(property_id)
        .execute(pool)
        .await?;

    Ok(())
}
