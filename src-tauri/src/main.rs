#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayEvent, SystemTrayMenu};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new().add_item(hide).add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        match window.is_visible() {
                            Ok(flag) => match flag {
                                true => {
                                    window.hide().unwrap();
                                    item_handle.set_title("Show").unwrap();
                                }
                                false => {
                                    window.show().unwrap();
                                    item_handle.set_title("Hide").unwrap();
                                }
                            },
                            Err(e) => println!("Error {:?}", e),
                        }
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
