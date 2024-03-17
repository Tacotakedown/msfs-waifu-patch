use crate::copy_and_replace::copy_and_replace_file;
use crate::paths::Paths;

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

mod copy_and_replace;
mod paths;

fn main() {
    let mut official_input = String::new();
    let mut exe_input = String::new();

    let exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error getting current executable path: {}", e);
            return;
        }
    };

    let exe_directory = exe_path.parent().unwrap_or_else(|| {
        eprintln!("Error getting parent directory of executable.");
        std::process::exit(1);
    });
    let project_root_path = exe_directory
        .parent()
        .and_then(|parent| parent.parent())
        .unwrap_or_else(|| {
            eprintln!("Error getting project root directory.");
            std::process::exit(1);
        });

    let patch_directory = project_root_path.join("patch");
    if !patch_directory.exists() {
        match fs::create_dir(&patch_directory) {
            Ok(_) => println!("Created 'patch' directory."),
            Err(e) => {
                eprintln!("Error creating 'patch' directory: {}", e);
                std::process::exit(1);
            }
        }
    }

    const EXCLUDED_FILES: [&str; 1] = ["readme.md"];
    let patch_files: Vec<PathBuf> = match fs::read_dir(&patch_directory) {
        Ok(entries) => entries
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    if e.file_type().map_or(false, |ft| ft.is_file()) {
                        let file_name = e.file_name().to_string_lossy().to_lowercase();
                        if !EXCLUDED_FILES.iter().any(|&f| file_name.contains(f)) {
                            Some(e.path())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
            })
            .collect(),
        Err(e) => {
            eprintln!("Error reading 'patch' directory: {}", e);
            std::process::exit(1);
        }
    };

    println!("Paste your full MSFS Official folder path (make sure to include either Steam or the MSStore equivelent eg. C:\\Users\\Taco\\AppData\\Roaming\\Microsoft Flight Simulator\\Packages\\Official\\Steam");
    io::stdin()
        .read_line(&mut official_input)
        .expect("Failed to read official input.");

    println!("Paste your MSFS exe path, this should be different from the last one, if you dont have a config issue eg. C:\\Program Files (x86)\\Steam\\steamapps\\common\\MicrosoftFlightSimulator");
    io::stdin()
        .read_line(&mut exe_input)
        .expect("Failed to read exe input.");

    official_input = official_input.trim().replace("\r\n", "");
    exe_input = exe_input.trim().replace("\r\n", "");

    for file in patch_files {
        let file_name = file.file_name().unwrap().to_string_lossy();
        let source_path = file.to_str().unwrap();
        let destination_path = match file_name.to_lowercase().as_str() {
            "flightplanning.css" => format!("{}{}", &official_input, Paths::FLIGHT_PLANNING),
            "flightplanning.html" => format!("{}{}", &official_input, Paths::FLIGHT_PLANNING_HTML),
            "flowbar.css" => format!("{}{}", &exe_input, Paths::FLOW_BAR),
            "flowbutton.css" => format!("{}{}", &exe_input, Paths::FLOW_BUTTON),
            "widgetmenubutton.css" => format!("{}{}", &exe_input, Paths::WIDGET_MENU_BUTTON),
            "widgetshomepage.xml" => format!("{}{}", &official_input, Paths::WIDGETS_HOMEPAGE),
            "widgetsprofile.xml" => format!("{}{}", &official_input, Paths::WIDGETS_PROFILE),
            _ => {
                eprintln!("Unknown file {}", file_name);
                continue;
            }
        };

        match copy_and_replace_file(source_path, &destination_path) {
            Ok(_) => println!(
                "File {} copied and replaced successfully.",
                destination_path
            ),
            Err(e) => eprintln!("Error copying file {}: {}", destination_path, e),
        }
    }
}
