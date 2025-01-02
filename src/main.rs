use clap::Parser;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser)]
struct Args {
    /// Path to the directory to organize
    directory: String,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.directory);

    if !path.exists() {
        eprintln!("Error: Directory does not exist.");
        return;
    }

    for file in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if file.file_type().is_file() {
            let extension = file.path().extension().and_then(|e| e.to_str()).unwrap_or("unknown_file_type");
            let target_directory = path.join(extension);

            //  move the file
            fs::create_dir_all(&target_directory).unwrap();
            fs::rename(file.path(), target_directory.join(file.file_name())).unwrap();
            println!("Moved {:?} to {:?}", file.path(), target_directory.join(file.file_name()));
        }
    }
}

