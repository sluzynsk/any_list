#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::image::Image;
use tauri::Manager;
use tauri::menu::{MenuBuilder, MenuItem};
use tauri::tray::TrayIconBuilder;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let toggle_item = Arc::new(Mutex::new(None));
    let toggle_item_clone = toggle_item.clone();

    tauri::Builder::default()
        .setup(move |app| {
            let title_i = MenuItem::with_id(app, "title", "Any List", false, None::<&str>)?;
            let toggle_i = MenuItem::with_id(app, "toggle", "Toggle", true, None::<&str>)?;
            *toggle_item.lock().unwrap() = Some(toggle_i.clone());
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let tray_menu = MenuBuilder::new(app)
                .item(&title_i)
                .separator()
                .item(&toggle_i)
                .item(&quit_i)
                .build()?;
            let _ = TrayIconBuilder::with_id("tray-1")
                .menu(&tray_menu)
                .tooltip("Any List Desktop")
                .menu_on_left_click(false)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "toggle" => {
                        if let Some(webview_window) = app.get_webview_window("main") {
                            let new_title = if webview_window.is_visible().unwrap_or_default() {
                                let _ = webview_window.hide();
                                "Show"
                            } else {
                                let _ = webview_window.show();
                                let _ = webview_window.set_focus();
                                "Hide"
                            };
                            toggle_i.set_text(new_title).unwrap();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => (),
                })
                .icon(Image::from_bytes(include_bytes!("../icons/icon.png"))?)
                .icon_as_template(true)
                .build(app)?;

            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            let windows = app.webview_windows();

            windows
                .values()
                .next()
                .expect("Sorry, no window found")
                .show()
                .expect("Can't Show Window");

            windows
                .values()
                .next()
                .expect("Sorry, no window found")
                .set_focus()
                .expect("Can't Bring Window to Focus");
        }))
        .on_window_event(move |event_window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if let Some(toggle_menu_item) = &*toggle_item_clone.lock().unwrap() {
                    let _ = toggle_menu_item.set_text("Show");
                }

   
                event_window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("Error while running application");
}
