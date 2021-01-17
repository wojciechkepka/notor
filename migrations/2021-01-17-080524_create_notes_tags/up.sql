CREATE TABLE notes
(
    note_id SERIAL PRIMARY KEY NOT NULL,
    title VARCHAR(256) NOT NULL,
    content TEXT
);
CREATE TABLE tags
(
    tag_id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(64) NOT NULL
);
CREATE TABLE notes_tags
(
    note_tag_id SERIAL PRIMARY KEY NOT NULL,
    note_id INT NOT NULL,
    tag_id INT NOT NULL,
    CONSTRAINT fk_note
        FOREIGN KEY(note_id)
            REFERENCES notes(note_id),
    CONSTRAINT fk_tag
        FOREIGN KEY(tag_id)
            REFERENCES tags(tag_id)
);
