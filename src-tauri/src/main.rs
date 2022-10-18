#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use url::Url;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            match app.get_cli_matches() {
                Ok(matches) => match matches.subcommand {
                    Some(subcom) => match subcom.name.as_str() {
                        "create-window" => {
                            let title = &subcom.matches.args.get("title").clone().unwrap().value;
                            let label = &subcom.matches.args.get("label").clone().unwrap().value;
                            let url = &subcom.matches.args.get("url").clone().unwrap().value;
                            println!("title {:?}", title);
                            println!("url {:?}", url);
                            println!("label {:?}", label);

                            let proper_url = Url::parse(&url.as_str().unwrap()).unwrap();

                            tauri::WindowBuilder::new(
                                app,
                                label.as_str().unwrap(),
                                tauri::WindowUrl::External(proper_url),
                            )
                            .title(title.as_str().unwrap())
                            .resizable(true)
                            .visible(true)
                            .transparent(true)
                            .decorations(false)
                            .position(0.0, 0.0)
                            .inner_size(800.0, 800.0)
                            .focus()
                            .build()?;
                        }
                        _ => println!(
                            "No matching subcommand found. (Did you mean 'create-window'?)"
                        ),
                    },
                    _ => println!("No subcommand passed. (Did you mean 'create-window'?)"),
                },
                Err(_) => {}
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
