-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
                                     user_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                                     first_name VARCHAR NOT NULL,
                                     last_name VARCHAR NOT NULL,
                                     username VARCHAR NOT NULL UNIQUE,
                                     email VARCHAR NOT NULL UNIQUE,
                                     password VARCHAR NOT NULL UNIQUE,
                                     created_at TIMESTAMP WITH TIME ZONE  NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                     updated_at TIMESTAMP WITH TIME ZONE  NOT NULL DEFAULT CURRENT_TIMESTAMP
);