DROP TABLE notes, tags, notes_tags CASCADE;

CREATE TABLE IF NOT EXISTS notes
(
    id          INT GENERATED ALWAYS AS IDENTITY,
    user_id     INT NOT NULL,
    created     TIMESTAMP NOT NULL,
    title       VARCHAR(256) NOT NULL,
    content     TEXT,

    PRIMARY KEY(id),

    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS tags
(
    id          INT GENERATED ALWAYS AS IDENTITY,
    user_id     INT NOT NULL,
    name        VARCHAR(64) NOT NULL,

    PRIMARY KEY(id),
    
    CONSTRAINT fk_user
        FOREIGN KEY(user_id)
            REFERENCES users(id)
);


CREATE TABLE IF NOT EXISTS notes_tags
(
    note_id     INT,
    tag_id      INT,

    CONSTRAINT fk_notes
        FOREIGN KEY(note_id)
            REFERENCES notes(id),

    CONSTRAINT fk_tags
        FOREIGN KEY(tag_id)
            REFERENCES tags(id)
);
