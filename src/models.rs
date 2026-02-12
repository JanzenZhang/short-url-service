use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUrlRequest {
    #[validate(url)]
    #[schema(example = "https://www.google.com")]
    pub url: String,
    #[schema(example = "custom123", nullable)]
    #[validate(length(min = 3, max = 20))]
    pub custom_code: Option<String>,
    #[schema(example = "2025-12-31T23:59:59Z", nullable)]
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UrlResponse {
    pub short_code: String,
    pub original_url: String,
    pub short_url: String,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct UrlRecord {
    pub id: String,
    pub original_url: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, ToSchema)]
pub struct VisitStats {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub visited_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StatsResponse {
    pub url: String,
    pub total_visits: i64,
    pub visits: Vec<VisitStats>,
}
