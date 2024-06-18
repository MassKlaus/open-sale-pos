-- Your SQL goes here
CREATE TABLE sales (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    transaction_date DATETIME NOT NULL,
    total_amount DECIMAL(10, 2) NOT NULL,
    tax_amount DECIMAL(10, 2) NOT NULL,
    discount_amount DECIMAL(10, 2) NOT NULL,
    payment_method VARCHAR(255) NOT NULL,
    customer_id INT,
    employee_id INT NOT NULL,
    FOREIGN KEY (customer_id) REFERENCES customers(id),
    FOREIGN KEY (employee_id) REFERENCES employees(id)
);