use clap::Parser;
use std::fs;
use std::path::{Path};
use walkdir::WalkDir;

#[derive(Parser)]
struct Args {
    /// Path to the directory to organize
    directory: String,

    /// Initial letter or prefix for filtering files (optional)
    #[arg(short = 'n', long)]
    initial: Option<String>,

    /// File extension for filtering (e.g., .pdf, .txt) (optional)
    #[arg(short, long)]
    extension: Option<String>,
}

fn clean_empty_directories(path: &Path) {
    // Walk through the directories and remove any empty directories
    for dir in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if dir.file_type().is_dir() {
            // If directory is empty, remove it
            if fs::read_dir(&dir.path()).unwrap().count() == 0 {
                println!("Removing empty directory: {:?}", dir.path());
                fs::remove_dir(dir.path()).unwrap();
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.directory);

    if !path.exists() {
        eprintln!("Error: Directory does not exist.");
        return;
    }

    // Set to track already organized files
    let mut organized_files = Vec::new();

    // First pass to organize by extensions, then by initials
    for file in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if file.file_type().is_file() {
            let file_name = file.file_name().to_string_lossy();
            let extension = file.path().extension().and_then(|e| e.to_str()).unwrap_or("unknown_file_type");

            // Check if filtering is applied (initial and extension)
            let matches_initial = if let Some(ref initial) = args.initial {
                file_name.to_lowercase().starts_with(&initial.to_lowercase())
            } else {
                true
            };

            let matches_extension = if let Some(ref ext) = args.extension {
                extension == ext.trim_start_matches('.')
            } else {
                true
            };

            // If both filters match, move the file
            if matches_initial && matches_extension {
                // Create the extension directory
                let extension_directory = path.join(extension);

                // Create a descriptive name for the inner directory
                let initial_directory_name = format!("{}_files_with_{}_initial", extension, args.initial.clone().unwrap_or_default());

                // Now create the initial directory inside the extension directory
                let initial_directory = extension_directory.join(initial_directory_name);

                // Generate the target file path
                let target_file_path = initial_directory.join(file.file_name());

                // If the file is already in the correct directory, skip it
                if file.path().parent() == Some(&initial_directory) {
                    println!("Skipping {:?}, already organized.", file.path());
                    organized_files.push(file.path().to_path_buf()); // Track the file
                    continue;
                }

                // Move the file if it's not already in the target directory
                fs::create_dir_all(&initial_directory).unwrap();
                fs::rename(file.path(), &target_file_path).unwrap();
                println!(
                    "Moved {:?} to {:?}",
                    file.path(),
                    &target_file_path
                );
                organized_files.push(target_file_path); // Track the moved file
            }
        }
    }

    // Second pass to organize based on initials (ignoring already organized files)
    for file in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if file.file_type().is_file() {
            let file_name = file.file_name().to_string_lossy();
            let extension = file.path().extension().and_then(|e| e.to_str()).unwrap_or("unknown_file_type");

            let matches_initial = if let Some(ref initial) = args.initial {
                file_name.to_lowercase().starts_with(&initial.to_lowercase())
            } else {
                true
            };

            let matches_extension = if let Some(ref ext) = args.extension {
                extension == ext.trim_start_matches('.')
            } else {
                true
            };

            // Check if it's already organized (if it's already in the target directory)
            let extension_directory = path.join(extension);
            let initial_directory_name = format!("{}_files_with_{}_initial", extension, args.initial.clone().unwrap_or_default());
            let initial_directory = extension_directory.join(initial_directory_name);
            let target_file_path = initial_directory.join(file.file_name());

            if matches_initial && matches_extension && !organized_files.contains(&file.path().to_path_buf()) {
                fs::create_dir_all(&initial_directory).unwrap();
                fs::rename(file.path(), &target_file_path).unwrap();
                println!(
                    "Moved {:?} to {:?}",
                    file.path(),
                    &target_file_path
                );
                organized_files.push(target_file_path);
            } else {
                println!("Skipping {:?}, already organized.", file.path());
            }
        }
    }

    // Cleanup empty directories after organizing files
    clean_empty_directories(path);
}

