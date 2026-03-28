-- migrations/20260328000001_user_engagement_schema.sql
ALTER TABLE users ADD COLUMN role VARCHAR(10) NOT NULL DEFAULT 'user';
ALTER TABLE users ADD COLUMN totp_setup_completed BOOLEAN NOT NULL DEFAULT FALSE;

-- Existing admins should have their setup completed
UPDATE users SET role = 'admin', totp_setup_completed = TRUE;

CREATE TABLE user_invites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role VARCHAR(10) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_by UUID REFERENCES users(id),
    used_at TIMESTAMPTZ
);

CREATE TABLE user_visits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    pub_id UUID NOT NULL REFERENCES pubs(id),
    visit_date DATE NOT NULL,
    notes TEXT,
    UNIQUE (user_id, pub_id, visit_date)
);

CREATE TABLE suggested_updates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pub_id UUID NOT NULL REFERENCES pubs(id),
    user_id UUID NOT NULL REFERENCES users(id),
    status VARCHAR(10) NOT NULL DEFAULT 'pending',
    suggested_data JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    processed_at TIMESTAMPTZ,
    processed_by UUID REFERENCES users(id)
);

CREATE TABLE pub_photos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pub_id UUID NOT NULL REFERENCES pubs(id),
    user_id UUID REFERENCES users(id),
    flickr_id TEXT,
    image_url TEXT NOT NULL,
    owner_name TEXT NOT NULL,
    license_type TEXT NOT NULL,
    license_url TEXT NOT NULL,
    original_url TEXT NOT NULL,
    is_cc_licensed BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
