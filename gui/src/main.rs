use std::path::Path;

use recursive_hash_calculator_core::{hash_strategy::HashStrategy, hasher, report_type::ReportType};
use rfd::FileDialog;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;

    window.on_input_path_button_clicked({
        let window = window.as_weak().unwrap();
        move || window.set_input_path(open_directory_input().into())
    });
    window.on_output_path_button_clicked({
        let window = window.as_weak().unwrap();
        move || window.set_output_path(open_directory_input().into())
    });

    window.on_calculate_button_clicked({
        let weak_window = window.as_weak();

        move || {
            let window = weak_window.unwrap();
            let input_path = window.get_input_path().to_string();
            let output_path = window.get_output_path().to_string();

            let output_path = Path::new(&output_path)
                .join(report_filename())
                .to_str()
                .unwrap()
                .to_owned();

            window.set_error_message("".into());

            if input_path.is_empty() {
                window.set_error_message("Please select an input directory.".into());
                return;
            }

            if output_path.is_empty() {
                window.set_error_message("Please select a directory for the report to be saved.".into());
                return;
            }

            window.set_buttons_enabled(false);

            tokio::spawn(calculate_hashes(weak_window.clone(), input_path, output_path));
        }
    });

    window.run()
}

fn open_directory_input() -> String {
    let directory = FileDialog::new().pick_folder();

    match directory {
        Some(path) => path.to_str().unwrap_or_default().to_owned(),
        None => String::default(),
    }
}

async fn calculate_hashes(window: slint::Weak<MainWindow>, input_path: String, output_path: String) {
    let result = hasher::process(input_path, HashStrategy::Sha256, ReportType::Csv(output_path)).await;

    println!("Elapsed: {:?}", result.elapsed_time);

    window
        .upgrade_in_event_loop(move |window| {
            window.set_buttons_enabled(true);
        })
        .unwrap();
}

fn report_filename() -> String {
    chrono::Local::now().format("%Y-%m-%d_%H%M%S.csv").to_string()
}
