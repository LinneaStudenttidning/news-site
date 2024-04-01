DO $$ BEGIN
    CREATE TYPE creator_role AS ENUM('publisher', 'writer');
        EXCEPTION WHEN DUPLICATE_OBJECT THEN RAISE NOTICE '"creator_role" exists, skipping...';
END $$;

DO $$ BEGIN
    CREATE TYPE text_type AS ENUM('article', 'coverage', 'opinion', 'other');
        EXCEPTION WHEN DUPLICATE_OBJECT THEN RAISE NOTICE '"text_type" exists, skipping...';
END $$;

DO $$ BEGIN
    CREATE TYPE text_lang AS ENUM('english', 'swedish');
        EXCEPTION WHEN DUPLICATE_OBJECT THEN RAISE NOTICE '"text_type" exists, skipping...';
END $$;

CREATE TABLE IF NOT EXISTS creators (
    display_name varchar(128) NOT NULL,
    username varchar(64) NOT NULL PRIMARY KEY,
    password varchar(128) NOT NULL,
    biography text NOT NULL,
    joined_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    role creator_role NOT NULL DEFAULT 'writer'
);

CREATE SEQUENCE IF NOT EXISTS articles_id_seq;

CREATE TABLE IF NOT EXISTS articles (
    id serial NOT NULL PRIMARY KEY,
    title varchar(256) NOT NULL,
    /* Reference to the username of the creator. */
    author varchar(64) NOT NULL,
    content text NOT NULL,
    text_type text_type NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tags varchar(64) [] NOT NULL DEFAULT ARRAY[]::integer[],
    -- Generate a search vector for title and content. It should prioritize Swedish over English.
    search_vec tsvector GENERATED ALWAYS AS (
        setweight(
            to_tsvector('swedish', title || ' ' || content),
            'A'
        ) || setweight(
            to_tsvector('english', title || ' ' || content),
            'B'
        )
    ) STORED
);

CREATE TABLE IF NOT EXISTS images (
    id uuid NOT NULL PRIMARY KEY,
    author text NOT NULL,
    description text,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tags varchar(64) [] NOT NULL DEFAULT ARRAY[]::integer[]
);

CREATE INDEX IF NOT EXISTS idx_creators_username ON creators (username);

CREATE INDEX IF NOT EXISTS idx_articles_id ON articles (id);
CREATE INDEX IF NOT EXISTS idx_articles_title ON articles (title);
CREATE INDEX IF NOT EXISTS idx_articles_tags ON articles USING GIN (tags);
CREATE INDEX IF NOT EXISTS idx_articles_search ON articles USING GIN (search_vec);

CREATE INDEX IF NOT EXISTS idx_images_tags ON images USING GIN (tags);