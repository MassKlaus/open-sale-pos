-- Your SQL goes here
CREATE TABLE suppliers (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    contact_name VARCHAR(255),
    email VARCHAR(255),
    phone VARCHAR(255),
    address TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);