#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use verilinq::{check_page_links_impl, LinkResult};

#[tauri::command]
async fn check_page_links(url: String) -> Result<Vec<LinkResult>, String> {
  check_page_links_impl(url).await
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![check_page_links])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
