use warp::{http::StatusCode, Rejection, Reply};

use super::db::DBHealth;

pub async fn health_handler(db_access: impl DBHealth) -> Result<impl Reply, Rejection> {
    db_access.health().await?;
    Ok(StatusCode::OK)
}
