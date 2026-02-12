# 🔗 SwiftLink - Rust URL Shortener

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)
![Axum](https://img.shields.io/badge/framework-axum-red.svg)

A high-performance, self-hosted URL shortener service built with Rust. It features a modern, responsive web interface, comprehensive visit statistics, and a robust REST API.

## ✨ Features

- **🚀 High Performance**: Built on the blazing fast `Axum` web framework and `Tokio` runtime.
- **🎨 Modern UI**: Clean, responsive web interface with:
    - **Quick Shorten**: Generate random or custom short codes.
    - **Smart Lookup**: Query short codes to view the original URL and stats without redirecting.
    - **QR Codes**: Instant SVG QR code generation for every link.
    - **Tabbed Interface**: Seamless switching between creation and lookup modes.
- **📊 Analytics**: Track detailed visit statistics including:
    - Total visit counts.
    - Visitor IP addresses (respects `X-Forwarded-For`).
    - User-Agent strings.
    - Timestamped visit logs.
- **🔗 Link Management**:
    - **Custom Aliases**: User-defined short codes (e.g., `/my-promo`).
    - **Expiration**: Set optional expiration dates for links.
- **📚 API Documentation**: Interactive Swagger UI generated via `Utoipa`.

## 🛠️ Tech Stack

- **Language:** Rust
- **Web Framework:** Axum 0.8
- **Database:** SQLite (via SQLx)
- **Async Runtime:** Tokio
- **API Docs:** Utoipa (Swagger UI)
- **Serialization:** Serde
- **Frontend:** HTML5, CSS3 (Internal styling), Vanilla JS

## 🚀 Getting Started

### Prerequisites

- **Rust**: Latest stable version.
- **SQLite**: (Optional) The application manages the database file automatically.

### Installation & Run

1.  **Clone the repository**
    ```bash
    git clone https://github.com/JanzenZhang/short-url-service.git
    cd short-url-service
    ```

2.  **Build the project**
    ```bash
    cargo build --release
    ```

3.  **Run the server**
    ```bash
    cargo run
    ```
    The server will start listening on `http://127.0.0.1:3000`.

4.  **Access the Application**
    - **Web UI:** Open `http://127.0.0.1:3000` in your browser.
    - **API Docs:** Visit `http://127.0.0.1:3000/swagger-ui`.

## 🔌 API Endpoints

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `POST` | `/shorten` | Create a new short link. |
| `GET` | `/{code}` | Redirect to the original URL. |
| `GET` | `/stats/{code}` | Retrieve stats and original URL. |
| `GET` | `/qr/{code}` | Get the QR code image (SVG). |

### Example Request

**Shorten a URL:**
```bash
curl -X POST http://127.0.0.1:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://www.rust-lang.org",
    "custom_code": "rust",
    "expires_at": "2026-12-31T23:59:59Z"
  }'
```

**Get Stats:**
```bash
curl http://127.0.0.1:3000/stats/rust
```

## ⚙️ Configuration

The application uses a `.env` file for configuration.

### Example `.env` File
```dotenv
DATABASE_URL=sqlite:shortener.db?mode=rwc
RUST_LOG=info
```

- `DATABASE_URL`: Connection string for SQLite. `mode=rwc` ensures the database file is created if it doesn't exist.
- `RUST_LOG`: Log level (default: `info` or `debug`).

## 📄 License

This project is licensed under the MIT License.
