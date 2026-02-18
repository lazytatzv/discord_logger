-- Add migration script here
CREATE TABLE message_logs (
  id SERIAL PRIMARY KEY,
  guild_id BIGINT,
  channel_id BIGINT,
  author_id BIGINT,
  content TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW()
);
