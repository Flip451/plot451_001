CREATE TABLE directories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    parent_id INTEGER,
    CONSTRAINT parent_id
        FOREIGN KEY (parent_id) REFERENCES directories(id)
        ON DELETE CASCADE
);

CREATE TABLE columns (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    directory_id INTEGER NOT NULL,
    CONSTRAINT directory_id
        FOREIGN KEY (directory_id) REFERENCES directories(id)
        ON DELETE CASCADE
);

CREATE TABLE cells (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    value REAL,
    column_id INTEGER NOT NULL,
    CONSTRAINT column_id
        FOREIGN KEY (column_id) REFERENCES columns(id)
        ON DELETE CASCADE
);

CREATE TABLE tables (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE table_columns (
    table_id INTEGER NOT NULL,
    column_id INTEGER NOT NULL,
    CONSTRAINT table_id
        FOREIGN KEY (table_id) REFERENCES tables(id)
        ON DELETE CASCADE,
    CONSTRAINT column_id
        FOREIGN KEY (column_id) REFERENCES columns(id)
        ON DELETE CASCADE
);