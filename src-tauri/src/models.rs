use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::products)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct Product {
    id: i32,
    name: String,
    description: Option<String>,
    barcode: String,
    price: f64,
    tax_rate: f64,
    supplier_id: Option<i32>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::product_tag_categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ProductTagCategory {
    pub product_id: i32,
    pub tag_category_id: i32,
}

#[derive(Debug, Clone, Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tag_categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TagCategory {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::product_tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ProductTag {
    pub product_id: i32,
    pub tag_id: i32,
}

#[derive(Debug, Clone, Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tag {
    pub tag_category_id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Customer {
    id: i32,
    first_name: String,
    last_name: String,
    email: Option<String>,
    phone: Option<String>,
    address: Option<String>,
    loyalty_points: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::employees)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Employee {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    phone: Option<String>,
    role: String,
    password_hash: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::sales)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Sale {
    id: i32,
    transaction_date: DateTime<Utc>,
    total_amount: f64,
    tax_amount: f64,
    discount_amount: f64,
    payment_method: String,
    customer_id: Option<i32>,
    employee_id: i32,
}

#[derive(Debug, Clone, Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::sale_items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SaleItem {
    id: i32,
    sale_id: i32,
    product_id: i32,
    quantity: i32,
    unit_price: f64,
    total_price: f64,
}

#[derive(Debug, Clone, Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::inventories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Inventory {
    id: i32,
    product_id: i32,
    stock_quantity: i32,
    reorder_level: i32,
    supplier_id: Option<i32>,
    last_restock_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::suppliers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Supplier {
    id: i32,
    name: String,
    contact_name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    address: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Role {
    id: i32,
    name: String,
    permissions: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

