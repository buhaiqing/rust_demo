#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

/// Pure function for greeting - separated for testability
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn greet_command(name: &str) -> String {
    greet(name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet_command])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_title("Rust Demo").unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_with_normal_name() {
        assert_eq!(greet("World"), "Hello, World!");
    }

    #[test]
    fn test_greet_with_empty_string() {
        assert_eq!(greet(""), "Hello, !");
    }

    #[test]
    fn test_greet_with_special_characters() {
        assert_eq!(greet("Rust & Tauri"), "Hello, Rust & Tauri!");
    }

    #[test]
    fn test_greet_with_unicode() {
        assert_eq!(greet("世界"), "Hello, 世界!");
    }
}
