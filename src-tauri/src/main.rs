#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn compute(previous_op: String, current_op: String, operator: String) -> Result<f64, &'static str> {
    let current: f64 = match current_op.parse() {
        Ok(num) => num,
        Err(_) => return Err("Opps looks like you forgot something, try again!"),
    };
    let previous: f64 = match previous_op.parse() {
        Ok(num) => num,
        Err(_) => return Err("Opps looks like you forgot something, try again!"),
    };
    return match operator.as_str() {
        "+" => Ok(previous + current),
        "-" => Ok(previous - current),
        "/" => Ok(previous / current),
        "*" => Ok(previous * current),
        _ => Err("Opps cannot compute that one, try again!"),
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compute])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
