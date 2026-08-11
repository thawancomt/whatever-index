CREATE TABLE IF NOT EXISTS files  (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    extension VARCHAR(30) NOT NULL,
    mtime TEXT NOT NULL,
    indexed_at TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    size_bytes INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_files_path on files(path);
