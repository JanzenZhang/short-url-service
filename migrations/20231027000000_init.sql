CREATE TABLE IF NOT EXISTS urls (
    id TEXT PRIMARY KEY,
    original_url TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    expires_at DATETIME
);

CREATE TABLE IF NOT EXISTS visits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url_id TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    visited_at DATETIME NOT NULL,
    FOREIGN KEY(url_id) REFERENCES urls(id)
);
