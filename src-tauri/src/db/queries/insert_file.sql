INSERT INTO files (path, extension, mtime, indexed_at, content_hash, size_bytes)
VALUES (?1, ?2, ?3, ?4, ?5, ?6)
ON CONFLICT(path) DO UPDATE SET
    mtime=excluded.mtime,
    content_hash=excluded.content_hash,
    indexed_at=excluded.indexed_at,
    size_bytes=excluded.size_bytes
