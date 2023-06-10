use std::fs;

fn main() {
    for i in 1..=10 {
        let file_name = format!("file_{}.md", i);
        fs::File::create(&file_name).unwrap_or_else(|err| {
            panic!("Failed to create file {}: {}", file_name, err);
        });
    }
}
