use walkdir;

fn main() {
    // recursively scan folder
    for entry in walkdir::WalkDir::new("src") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let file = entry.file_name().to_str().unwrap();
            if file.starts_with("._") || file.ends_with(".rs"){
                println!("Found file: {}", file);
            }
        }
    }
}
