use walkdir;

fn main() {
    for entry in walkdir::WalkDir::new("D:\\Music\\Samplepacks\\") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file = entry.file_name().to_str().unwrap();
            if file.starts_with("._") {
                println!("Found file: {}", file);
                std::fs::remove_file(entry.path()).unwrap();
                println!("Removed file: {}", file);
            }
        }
    }
}
