CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE pubs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    address TEXT,
    town VARCHAR(100),
    county VARCHAR(100),
    postcode VARCHAR(20),
    closed BOOLEAN DEFAULT FALSE,
    location GEOGRAPHY(POINT, 4326),
    untappd_id VARCHAR(100),
    google_maps_id VARCHAR(255),
    whatpub_id VARCHAR(255),
    rgl_id VARCHAR(255),
    untappd_verified BOOLEAN DEFAULT FALSE,
    last_seen DATE DEFAULT CURRENT_DATE
);

CREATE TABLE gbg_history (
    id SERIAL PRIMARY KEY,
    pub_id UUID REFERENCES pubs(id) ON DELETE CASCADE,
    year INTEGER NOT NULL,
    UNIQUE(pub_id, year)
);
