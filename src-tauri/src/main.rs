#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use url::Url;
// use std::path::PathBuf;
// use tauri::Icon;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            match app.get_cli_matches() {
                Ok(matches) => match matches.subcommand {
                    Some(subcom) => match subcom.name.as_str() {
                        "create-window" => {
                            println!("matches {:?}", subcom.matches);

                            let title = &subcom.matches.args.get("title").clone().unwrap().value;
                            let label = &subcom.matches.args.get("label").clone().unwrap().value;
                            let url = &subcom.matches.args.get("url").clone().unwrap().value;
                            let icon_path = &subcom.matches.args.get("icon").clone().unwrap().value;
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
                            println!("icon_path {:?}", icon_path);
                            println!("focused {:?}", focused);
                            println!("transparent {:?}", transparent);
                            println!("decorations {:?}", decorations);

                            let proper_url = Url::parse(&url.as_str().unwrap()).unwrap();

                            tauri::WindowBuilder::new(
                                app,
                                label.as_str().unwrap(),
                                tauri::WindowUrl::External(proper_url),
                            )
                            .title(title.as_str().unwrap())
                            .resizable(true)
                            .visible(true)
                            .transparent(transparent.unwrap())
                            // .title_bar_style(tauri::TitleBarStyle::Transparent)
                            .decorations(decorations.unwrap())
                            .position(0.0, 0.0)
                            .focused(focused.unwrap())
                            .inner_size(800.0, 800.0)
                            .build()?;

                            // match icon_path.as_str() {
                            //     Some(path) => {
                            //         let i = Icon::File(PathBuf::from(path));
                            //         match win.icon(i) {
                            //             Ok(w) => {
                            //                 println!("Icon set.");
                            //             }
                            //             Err(e) => {
                            //                 println!("Error setting icon {}", e);
                            //             }
                            //         }
                            //     }
                            //     _ => {
                            //         println!("Icon path could not be unwrapped.");
                            //     }
                            // };


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
