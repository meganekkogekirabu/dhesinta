CREATE TABLE IF NOT EXISTS dictionaries (
    id CHAR(21) PRIMARY KEY NOT NULL UNIQUE,
    owner_id CHAR(21) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    visibility VARCHAR(8) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS entries (
    id CHAR(21) PRIMARY KEY NOT NULL UNIQUE,
    dictionary_id CHAR(21) NOT NULL,
    word TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (dictionary_id) REFERENCES dictionaries(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS entry_fields (
    entry_id CHAR(21) NOT NULL,
    field_key TEXT NOT NULL,
    field_value TEXT NOT NULL,
    PRIMARY KEY (entry_id, field_key),
    FOREIGN KEY (entry_id) REFERENCES entries(id) ON DELETE CASCADE
);
