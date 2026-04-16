CREATE TABLE IF NOT EXISTS conversions (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  filename TEXT NOT NULL,
  source_path TEXT NOT NULL,
  output_path TEXT,
  status TEXT NOT NULL DEFAULT 'pending',
  error_message TEXT,
  markdown_content TEXT,
  image_paths TEXT,
  file_size INTEGER,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_config (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
