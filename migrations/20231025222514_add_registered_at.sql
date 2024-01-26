ALTER TABLE app_users ADD COLUMN IF NOT EXISTS registered_at timestamptz NOT NULL DEFAULT NOW();
