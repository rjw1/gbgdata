-- migrations/20260328000002_passkeys_schema.sql
CREATE TABLE user_credentials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    credential_id BYTEA NOT NULL,
    public_key BYTEA NOT NULL,
    sign_count BIGINT NOT NULL DEFAULT 0,
    transports TEXT[],
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
