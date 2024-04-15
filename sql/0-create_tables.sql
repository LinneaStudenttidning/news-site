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
    display_name text NOT NULL,
    username text NOT NULL PRIMARY KEY,
    password text NOT NULL,
    biography text NOT NULL,
    joined_at timestamp with time zone  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    role creator_role NOT NULL DEFAULT 'writer'
);

CREATE SEQUENCE IF NOT EXISTS articles_id_seq;

CREATE TABLE IF NOT EXISTS articles (
    id serial NOT NULL PRIMARY KEY,
    is_published boolean NOT NULL DEFAULT false,
    marked_as_done boolean NOT NULL DEFAULT false,
    title text NOT NULL,
    title_slug TEXT NOT NULL,
    /* Reference to the username of the creator. */
    author text NOT NULL,
    lead_paragraph text NOT NULL,
    text_body text NOT NULL,
    text_type text_type NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tags text [] NOT NULL DEFAULT ARRAY[]::integer[],
    -- Generate a search vector for title and content. It should prioritize Swedish over English.
    search_vec tsvector GENERATED ALWAYS AS (
        setweight(
            to_tsvector('swedish', title || ' ' || lead_paragraph || ' ' || text_body),
            'A'
        ) || setweight(
            to_tsvector('english', title || ' ' || lead_paragraph || ' ' || text_body),
            'B'
        )
    ) STORED
);

CREATE TABLE IF NOT EXISTS images (
    id uuid NOT NULL PRIMARY KEY,
    author text NOT NULL,
    description text,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tags text [] NOT NULL DEFAULT ARRAY[]::integer[],
    -- Generate a search vector for title and content. It should prioritize Swedish over English.
    search_vec tsvector GENERATED ALWAYS AS (
        setweight(
            to_tsvector('swedish', description),
            'A'
        ) || setweight(
            to_tsvector('english', description),
            'B'
        )
    ) STORED
);

CREATE INDEX IF NOT EXISTS idx_creators_username ON creators (username);

CREATE INDEX IF NOT EXISTS idx_articles_id ON articles (id);
CREATE INDEX IF NOT EXISTS idx_articles_title ON articles (title);
CREATE INDEX IF NOT EXISTS idx_articles_tags ON articles USING GIN (tags);
CREATE INDEX IF NOT EXISTS idx_articles_search ON articles USING GIN (search_vec);

CREATE INDEX IF NOT EXISTS idx_images_tags ON images USING GIN (tags);
CREATE INDEX IF NOT EXISTS idx_images_search ON images USING GIN (search_vec);
