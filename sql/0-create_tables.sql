CREATE TYPE IF NOT EXISTS creator_role AS ENUM (
    'publisher',
    'writer',
);

CREATE TABLE IF NOT EXISTS creators (
    display_name varchar(128) NOT NULL,
    username     varchar(64)  NOT NULL PRIMARY KEY,
    password     varchar(128) NOT NULL,
    biography    text         NOT NULL,
    joined_at    timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    role         creator_role NOT NULL DEFAULT 'writer'
);

CREATE TABLE IF NOT EXISTS articles (
    id         serial        NOT NULL PRIMARY KEY,
    title      varchar(256)  NOT NULL,
    /* Reference to the username of the creator. */
    author     varchar(64)   NOT NULL,
    content    text          NOT NULL,
    published  boolean       NOT NULL DEFAULT false,
    created_at timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    edited_at  timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tags       varchar(64)[] NOT NULL DEFAULT ARRAY[]::integer[]
);

CREATE INDEX IF NOT EXISTS idx_creators_username ON creators (username);
CREATE INDEX IF NOT EXISTS idx_articles_id       ON articles (id);
CREATE INDEX IF NOT EXISTS idx_articles_title    ON articles (title);
CREATE INDEX IF NOT EXISTS idx_articles_tags     ON articles USING GIN (tags);

/* TODO: Implement full text search (FTS) */