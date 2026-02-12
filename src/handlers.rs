use crate::{
    error::AppError,
    models::{CreateUrlRequest, StatsResponse, UrlRecord, UrlResponse, VisitStats},
    utils::generate_short_code,
};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
    Json,
};
use chrono::Utc;
use qrcode::QrCode;
use qrcode::render::svg;
use sqlx::{Pool, Sqlite};
use validator::Validate;

type DbPool = Pool<Sqlite>;

#[utoipa::path(
    post,
    path = "/shorten",
    request_body = CreateUrlRequest,
    responses(
        (status = 201, description = "URL shortened successfully", body = UrlResponse),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "Custom code already exists")
    )
)]
pub async fn shorten_url(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUrlRequest>,
) -> Result<impl IntoResponse, AppError> {
    if let Err(_) = payload.validate() {
        return Err(AppError::InvalidUrl);
    }

    let code = if let Some(custom) = &payload.custom_code {
        // Check if exists
        let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM urls WHERE id = ?)")
            .bind(custom)
            .fetch_one(&pool)
            .await?;
        
        if exists {
            return Err(AppError::CodeAlreadyExists);
        }
        custom.clone()
    } else {
        // Generate random unique code
        let mut attempts = 0;
        loop {
            let candidate = generate_short_code(6);
            let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM urls WHERE id = ?)")
                .bind(&candidate)
                .fetch_one(&pool)
                .await?;
            if !exists {
                break candidate;
            }
            attempts += 1;
            if attempts > 10 {
                return Err(AppError::InternalServerError("Failed to generate unique code".into()));
            }
        }
    };

    let now = Utc::now();
    sqlx::query("INSERT INTO urls (id, original_url, created_at, expires_at) VALUES (?, ?, ?, ?)")
        .bind(&code)
        .bind(&payload.url)
        .bind(now)
        .bind(payload.expires_at)
        .execute(&pool)
        .await?;

    // Construct full short URL (assuming localhost for now, can be configured)
    let short_url = format!("http://localhost:3000/{}", code);

    Ok((
        axum::http::StatusCode::CREATED,
        Json(UrlResponse {
            short_code: code,
            original_url: payload.url,
            short_url,
            expires_at: payload.expires_at,
        }),
    ))
}

#[utoipa::path(
    get,
    path = "/{code}",
    params(
        ("code" = String, Path, description = "Short code")
    ),
    responses(
        (status = 307, description = "Redirect to original URL"),
        (status = 404, description = "URL not found"),
        (status = 410, description = "URL expired")
    )
)]
pub async fn redirect_url(
    State(pool): State<DbPool>,
    Path(code): Path<String>,
    headers: axum::http::HeaderMap,
) -> Result<Response, AppError> {
    let url_record: Option<UrlRecord> = sqlx::query_as("SELECT * FROM urls WHERE id = ?")
        .bind(&code)
        .fetch_optional(&pool)
        .await?;

    let url = match url_record {
        Some(u) => u,
        None => return Err(AppError::UrlNotFound),
    };

    if let Some(expires_at) = url.expires_at {
        if Utc::now() > expires_at {
            return Err(AppError::UrlNotFound); // Or 410 Gone
        }
    }

    // Record visit asynchronously (spawn task)
    let pool_clone = pool.clone();
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());
    
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    tokio::spawn(async move {
        let _ = sqlx::query("INSERT INTO visits (url_id, ip_address, user_agent, visited_at) VALUES (?, ?, ?, ?)")
            .bind(code)
            .bind(ip)
            .bind(user_agent)
            .bind(Utc::now())
            .execute(&pool_clone)
            .await;
    });

    Ok(Redirect::temporary(&url.original_url).into_response())
}

#[utoipa::path(
    get,
    path = "/stats/{code}",
    params(
        ("code" = String, Path, description = "Short code")
    ),
    responses(
        (status = 200, description = "Statistics", body = StatsResponse),
        (status = 404, description = "URL not found")
    )
)]
pub async fn get_stats(
    State(pool): State<DbPool>,
    Path(code): Path<String>,
) -> Result<Json<StatsResponse>, AppError> {
    // Check if URL exists first
    let url_record: Option<UrlRecord> = sqlx::query_as("SELECT * FROM urls WHERE id = ?")
        .bind(&code)
        .fetch_optional(&pool)
        .await?;

    let url = match url_record {
        Some(u) => u,
        None => return Err(AppError::UrlNotFound),
    };

    let visits: Vec<VisitStats> = sqlx::query_as("SELECT ip_address, user_agent, visited_at FROM visits WHERE url_id = ? ORDER BY visited_at DESC LIMIT 100")
        .bind(&code)
        .fetch_all(&pool)
        .await?;
    
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM visits WHERE url_id = ?")
        .bind(&code)
        .fetch_one(&pool)
        .await?;

    Ok(Json(StatsResponse {
        url: code,
        original_url: url.original_url,
        total_visits: total,
        visits,
    }))
}

#[utoipa::path(
    get,
    path = "/qr/{code}",
    params(
        ("code" = String, Path, description = "Short code")
    ),
    responses(
        (status = 200, description = "QR Code SVG image"),
        (status = 404, description = "URL not found")
    )
)]
pub async fn generate_qr(
    State(pool): State<DbPool>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
     let url_record: Option<UrlRecord> = sqlx::query_as("SELECT * FROM urls WHERE id = ?")
        .bind(&code)
        .fetch_optional(&pool)
        .await?;

    let _url = match url_record {
        Some(u) => u,
        None => return Err(AppError::UrlNotFound),
    };

    // Construct full short URL
    let short_url = format!("http://localhost:3000/{}", code);
    
    let code = QrCode::new(short_url).map_err(|_| AppError::InternalServerError("QR generation failed".into()))?;
    let image = code.render::<svg::Color>().build();

    Ok((
        [(axum::http::header::CONTENT_TYPE, "image/svg+xml")],
        image,
    ))
}
