-- Your SQL goes here
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    tag_category_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    FOREIGN KEY (tag_category_id) REFERENCES tag_categories(id)
);
