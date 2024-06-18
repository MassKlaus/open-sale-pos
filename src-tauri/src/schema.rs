// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        address -> Nullable<Text>,
        loyalty_points -> Integer,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    employees (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        phone -> Nullable<Text>,
        role -> Text,
        password_hash -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    inventories (id) {
        id -> Integer,
        product_id -> Integer,
        stock_quantity -> Integer,
        reorder_level -> Integer,
        supplier_id -> Nullable<Integer>,
        last_restock_date -> Nullable<TimestamptzSqlite>,
    }
}

diesel::table! {
    product_tag_categories (product_id, tag_category_id) {
        product_id -> Integer,
        tag_category_id -> Integer,
    }
}

diesel::table! {
    product_tags (product_id, tag_id) {
        product_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    products (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        barcode -> Text,
        price -> Double,
        tax_rate -> Double,
        supplier_id -> Nullable<Integer>,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    roles (id) {
        id -> Integer,
        name -> Text,
        permissions -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    sale_items (id) {
        id -> Integer,
        sale_id -> Integer,
        product_id -> Integer,
        quantity -> Integer,
        unit_price -> Double,
        total_price -> Double,
    }
}

diesel::table! {
    sales (id) {
        id -> Integer,
        transaction_date -> TimestamptzSqlite,
        total_amount -> Double,
        tax_amount -> Double,
        discount_amount -> Double,
        payment_method -> Text,
        customer_id -> Nullable<Integer>,
        employee_id -> Integer,
    }
}

diesel::table! {
    suppliers (id) {
        id -> Integer,
        name -> Text,
        contact_name -> Nullable<Text>,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        address -> Nullable<Text>,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    tag_categories (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        tag_category_id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(inventories -> products (product_id));
diesel::joinable!(inventories -> suppliers (supplier_id));
diesel::joinable!(product_tag_categories -> products (product_id));
diesel::joinable!(product_tags -> products (product_id));
diesel::joinable!(product_tags -> tags (tag_id));
diesel::joinable!(products -> suppliers (supplier_id));
diesel::joinable!(sale_items -> products (product_id));
diesel::joinable!(sale_items -> sales (sale_id));
diesel::joinable!(sales -> customers (customer_id));
diesel::joinable!(sales -> employees (employee_id));
diesel::joinable!(tags -> tag_categories (tag_category_id));

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    employees,
    inventories,
    product_tag_categories,
    product_tags,
    products,
    roles,
    sale_items,
    sales,
    suppliers,
    tag_categories,
    tags,
);
