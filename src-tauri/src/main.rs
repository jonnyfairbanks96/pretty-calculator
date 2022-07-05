#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod calculator;

#[derive(serde::Serialize)]
pub struct NewOperands {
    previous: String,
    current: String,
    operator: String,
}

#[tauri::command]
fn get_total(
    previous_op: String,
    current_op: String,
    operator: String,
) -> Result<NewOperands, &'static str> {
    return calculator::compute::total(previous_op, current_op, operator);
}

// #[tauri::command]
// fn add_number(
//     previous_op: String,
//     current_op: String,
//     operator: String,
// ) -> Result<f64, &'static str> {
//     return calculator::compute::total(previous_op, current_op, operator);
// }

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(if cfg!(target_os = "macos") {
            tauri::Menu::os_default(&context.package_info().name)
        } else {
            tauri::Menu::default()
        })
        .invoke_handler(tauri::generate_handler![get_total])
        .run(context)
        .expect("error while running tauri application");
}
