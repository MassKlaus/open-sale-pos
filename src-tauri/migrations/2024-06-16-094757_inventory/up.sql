-- Your SQL goes here
CREATE TABLE inventories (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    product_id INT NOT NULL,
    stock_quantity INT NOT NULL,
    reorder_level INT NOT NULL,
    supplier_id INT,
    last_restock_date DATETIME,
    FOREIGN KEY (product_id) REFERENCES products(id),
    FOREIGN KEY (supplier_id) REFERENCES suppliers(id)
);