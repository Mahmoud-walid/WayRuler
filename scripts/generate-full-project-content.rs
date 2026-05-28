use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const MAX_FILE_SIZE: u64 = 1024 * 1024 * 2; // 2MB

const IGNORE_DIRS: &[&str] = &[
    "node_modules",
    ".git",
    ".expo",
    "android",
    "dist",
    "build",
    "scripts",
    "postman-collections",
    "blueprints",
    "assets",
    "target",
];

const IGNORE_FILES: &[&str] = &[
    "full-project-content.md",
    ".env",
    "package-lock.json",
    "eslint.config.mts",
    "tsconfig.json",
    "README.md",
    "nodemon.json",
    ".gitignore",
    ".sentryclirc",
    "Cargo.lock",
];

fn show_loader(processed_count: usize) {
    print!("\r📦 Processing files: {}", processed_count);
    io::stdout().flush().unwrap();
}

fn get_all_files(dir_path: &Path, files: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue,
        };

        if IGNORE_DIRS.contains(&file_name) {
            continue;
        }

        let metadata = match fs::metadata(&path) {
            Ok(meta) => meta,
            Err(_) => continue,
        };

        if metadata.is_dir() {
            get_all_files(&path, files);
        } else {
            if IGNORE_FILES.contains(&file_name) {
                continue;
            }

            if metadata.len() > MAX_FILE_SIZE {
                continue;
            }

            files.push(path);
        }
    }
}

fn create_markdown_content(files: &[PathBuf], base_path: &Path) -> String {
    let mut result = String::new();
    let mut processed_count = 0;

    for file in files {
        processed_count += 1;
        show_loader(processed_count);

        let relative_path = file
            .strip_prefix(base_path)
            .unwrap_or(file)
            .display()
            .to_string();

        println!(" => 📄 {}\n", relative_path);

        let content = fs::read_to_string(file)
            .unwrap_or_else(|_| "[Could not read file]".to_string());

        result.push_str(&format!(
            "{}\n```\n{}\n```\n-----\n",
            relative_path, content
        ));
    }

    result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let folder_path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from("./")
    };

    let mut all_files: Vec<PathBuf> = Vec::new();

    get_all_files(&folder_path, &mut all_files);

    println!(
        "🚀 Found {} files. Starting...\n",
        all_files.len()
    );

    let markdown_content =
        create_markdown_content(&all_files, &folder_path);

    let output_path = PathBuf::from("full-project-content.md");

    fs::write(&output_path, markdown_content)
        .expect("Failed to write output file");

    println!(
        "\n\n✅ Done. Processed {} files.",
        all_files.len()
    );

    println!("📄 Output: {}", output_path.display());
}