#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use url::Url;

fn main() {
    use tauri_plugin_cli::CliExt;
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            match app.cli().matches() {
                Ok(matches) => match matches.subcommand {
                    Some(subcom) => match subcom.name.as_str() {
                        "create-window" => {
                            println!("matches {:?}", subcom.matches);

                            let title = &subcom.matches.args.get("title").clone().unwrap().value;
                            let label = &subcom.matches.args.get("label").clone().unwrap().value;
                            let url = &subcom.matches.args.get("url").clone().unwrap().value;
                            let focused = &subcom
                                .matches
                                .args
                                .get("focused")
                                .clone()
                                .unwrap()
                                .value
                                .as_bool();
                            let transparent = &subcom
                                .matches
                                .args
                                .get("transparent")
                                .clone()
                                .unwrap()
                                .value
                                .as_bool();
                            let decorations = &subcom
                                .matches
                                .args
                                .get("decorations")
                                .clone()
                                .unwrap()
                                .value
                                .as_bool();

                            println!("title {:?}", title);
                            println!("label {:?}", label);
                            println!("url {:?}", url);
                            println!("focused {:?}", focused);
                            println!("transparent {:?}", transparent);
                            println!("decorations {:?}", decorations);

                            let proper_url = Url::parse(&url.as_str().unwrap()).unwrap();

                            let win = tauri::WebviewWindowBuilder::new(
                                app,
                                label.as_str().unwrap(),
                                tauri::WebviewUrl::External(proper_url),
                            )
                            .title(title.as_str().unwrap())
                            .resizable(true)
                            .visible(true)
                            .transparent(transparent.unwrap())
                            // .title_bar_style(tauri::TitleBarStyle::Transparent)
                            .decorations(decorations.unwrap())
                            .position(0.0, 0.0)
                            .focused(focused.unwrap())
                            .inner_size(800.0, 800.0);

                            win.build()?;
                        }
                        _ => println!(
                            "No matching subcommand found. (Did you mean 'create-window'?)"
                        ),
                    },
                    _ => println!("No subcommand passed. (Did you mean 'create-window'?)"),
                },
                Err(e) => println!("{}", e),
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
