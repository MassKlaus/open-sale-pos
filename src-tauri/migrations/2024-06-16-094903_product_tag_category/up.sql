-- Your SQL goes here
CREATE TABLE product_tag_categories (
    product_id INT NOT NULL,
    tag_category_id INT NOT NULL,
    PRIMARY KEY (product_id, tag_category_id),
    FOREIGN KEY (product_id) REFERENCES products(id),
    FOREIGN KEY (tag_category_id) REFERENCES TagCategorys(id)
);