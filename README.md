# Rust URL Shortener Service

A high-performance URL shortener service built with Rust, Axum, SQLx, and SQLite.

## Features
- Create short links with random or custom codes.
- Redirect to original URLs.
- Track visit statistics (IP, User-Agent, Time).
- QR Code generation for short links.
- Link expiration support.
- API Documentation (Swagger UI).
- Simple Web Interface.

## Tech Stack
- **Framework:** Axum
- **Database:** SQLite (via SQLx)
- **Runtime:** Tokio
- **Documentation:** Utoipa (Swagger UI)

## Getting Started

### Prerequisites
- Rust (latest stable)
- SQLite (optional, `sqlx` handles the DB file)

### Running

1. **Install Dependencies & Build**
   ```bash
   cargo build
   ```

2. **Run the Server**
   ```bash
   cargo run
   ```
   The server will start at `http://localhost:3000`.

3. **Use the Web Interface**
   Open `http://localhost:3000` in your browser.

### API Endpoints

- **POST /shorten**: Create a short link.
  - Body: `{"url": "https://example.com", "custom_code": "optional", "expires_at": "optional_iso_date"}`
- **GET /{code}**: Redirect to original URL.
- **GET /stats/{code}**: Get visit statistics.
- **GET /qr/{code}**: Get QR code SVG.

### API Documentation
Visit `http://localhost:3000/swagger-ui` for interactive API docs.

## Configuration
- `.env` file contains `DATABASE_URL`.
- Default port is `3000`.
