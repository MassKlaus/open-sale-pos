// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use db::establish_connection;
use diesel::{ExpressionMethods, RunQueryDsl};
use models::TagCategory;

mod db;
mod models;
mod schema;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_tag_categories,
            create_tag_category
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_tag_categories() -> Vec<TagCategory> {
    use self::schema::tag_categories::dsl::*;

    let conn = &mut establish_connection();
    let result = tag_categories.load::<models::TagCategory>(conn).unwrap();
    result
}

#[tauri::command]
fn create_tag_category(category_name: &str) {
    use schema::tag_categories::dsl::*;

    let conn = &mut establish_connection();

    let res = diesel::insert_into(tag_categories)
        .values(name.eq(category_name))
        .execute(conn).unwrap();
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
