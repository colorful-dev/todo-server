-- Your SQL goes here
CREATE TABLE todo (
    id SERIAL PRIMARY KEY,
    content VARCHAR NOT NULL,
    complete BOOLEAN NOT NULL DEFAULT FALSE,
    -- Add a created_at column that is a timestamp with time zone
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    -- Add a updated_at column that is a timestamp with time zone
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
)
