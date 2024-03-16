use std::env;
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;

fn copy_and_replace_file(source_path: &str, destination_path: &str) -> io::Result<()> {
    let source_file = Path::new(source_path);

    if !source_file.exists() {
        return Err(Error::new(ErrorKind::NotFound, "Source file not found"));
    }

    let destination_file = Path::new(destination_path);
    if destination_file.exists() {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let backup_path = format!("{}.backup_{}", destination_path, timestamp);
        fs::copy(destination_path, &backup_path)?;
    }

    fs::copy(source_path, destination_path)?;

    Ok(())
}

fn main() {
    const WIDGET_MENU_BUTTON: &str =
        "\\Packages\\fs-base-ui\\html_ui\\Templates\\WidgetMenuButton\\WidgetMenuButton.css"; // steam (exe)
    const WIDGETS_HOMEPAGE: &str = "\\fs-base-ui-widgets\\Widgets\\WidgetsHomepage.xml"; // appdata
    const WIDGETS_PROFILE: &str = "\\fs-base\\widgets\\WidgetsProfile.xml"; // appdata
    const FLIGHT_PLANNING: &str =
        "\\fs-base-ui-pages\\html_ui\\Pages\\FlightPlanning\\FlightPlanning.css"; // appdata
    const FLIGHT_PLANNING_HTML: &str =
        "\\fs-base-ui-pages\\html_ui\\Pages\\lightPlanning\\FlightPlanning.html"; // appdata
    const FLOW_BUTTON: &str =
        "\\Packages\\fs-base-ui\\html_ui\\Templates\\FlowButton\\FlowButton.css"; // steam (exe install)
    const FLOW_BAR: &str = "\\Packages\\fs-base-onboarding\\html_ui\\ages\\FlowBar\\FlowBar.css"; // steam (exe)

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
            "flightplanning.css" => format!("{}{}", &official_input, FLIGHT_PLANNING),
            "flightplanning.html" => format!("{}{}", &official_input, FLIGHT_PLANNING_HTML),
            "flowbar.css" => format!("{}{}", &exe_input, FLOW_BAR),
            "flowbutton.css" => format!("{}{}", &exe_input, FLOW_BUTTON),
            "widgetmenubutton.css" => format!("{}{}", &exe_input, WIDGET_MENU_BUTTON),
            "widgetshomepage.xml" => format!("{}{}", &official_input, WIDGETS_HOMEPAGE),
            "widgetsprofile.xml" => format!("{}{}", &official_input, WIDGETS_PROFILE),
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
