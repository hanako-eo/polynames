CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid(),
    name STRING,
    -- profile_picture IMAGE
);
