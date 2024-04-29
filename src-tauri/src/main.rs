#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let hide = MenuItemBuilder::with_id("hide", "Hide").build(app)?;
            let tray_menu = MenuBuilder::new(app).items(&[&hide, &quit]).build()?;
            let _tray = TrayIconBuilder::new()
                .menu(&tray_menu)
                .icon(app.default_window_icon().cloned().unwrap())
                .tooltip("Any List Desktop")
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "hide" => {
                        if let Some(webview_window) = app.get_webview_window("main") {
                            match webview_window.is_visible() {
                                Ok(flag) => match flag {
                                    true => {
                                        webview_window.hide().unwrap();
                                        tray_menu
                                            .get(event.id().as_ref())
                                            .expect("Something bad has happened.")
                                            .as_menuitem()
                                            .expect("Something bad has happened.")
                                            .set_text("Show")
                                            .unwrap();
                                    }
                                    false => {
                                        webview_window.show().unwrap();
                                        webview_window.set_focus().unwrap();
                                        tray_menu
                                            .get(event.id().as_ref())
                                            .expect("Something bad has happened.")
                                            .as_menuitem()
                                            .expect("Something bad has happened.")
                                            .set_text("Hide")
                                            .unwrap();
                                    }
                                },
                                Err(e) => println!("Error {:?}", e),
                            }
                        }
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => (),
                })
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
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if let Some (menu) = window.menu() {
                    menu.get("hide")
                        .expect("Something bad has happened.")
                        .as_menuitem()
                        .expect("Something bad has happened.")
                        .set_text("Show")
                        .unwrap();
                }
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("Error while running application");
}
