CREATE TABLE site_settings (
    id SERIAL PRIMARY KEY,
    private_mode BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID REFERENCES users(id)
);

-- Initialize with default public state
INSERT INTO site_settings (private_mode) VALUES (FALSE);
