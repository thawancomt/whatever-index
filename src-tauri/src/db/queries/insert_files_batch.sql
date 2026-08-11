-- insert_files_batch.sql
INSERT OR REPLACE INTO files (path, extension, mtime, indexed_at, content_hash, size_bytes)
SELECT
    value ->> 'path',
    value ->> 'extension',
    CAST(value ->> 'mtime' AS INTEGER),
    CAST(value ->> 'indexed_at' AS INTEGER),
    value ->> 'content_hash',
    CAST(value ->> 'size_bytes' AS INTEGER)
FROM json_each(?1)
where true
ON CONFLICT(path) DO UPDATE SET
    mtime = excluded.mtime,
    content_hash = excluded.content_hash,
    indexed_at = excluded.indexed_at,
    size_bytes = excluded.size_bytes
