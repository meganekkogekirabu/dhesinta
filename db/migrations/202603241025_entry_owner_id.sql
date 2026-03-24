CREATE TABLE _entries (
    id CHAR(21) PRIMARY KEY NOT NULL UNIQUE,
    dictionary_id CHAR(21) NOT NULL,
    owner_id CHAR(21) NOT NULL,
    word TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (dictionary_id) REFERENCES dictionaries(id) ON DELETE CASCADE,
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
);

INSERT INTO _entries (id, dictionary_id, word, created_at, updated_at)
SELECT * FROM entries;

UPDATE _entries
SET owner_id = (
    SELECT d.owner_id
    FROM dictionaries d
    WHERE d.id = _entries.dictionary_id
);

DROP TABLE entries;

ALTER TABLE _entries RENAME TO entries;
