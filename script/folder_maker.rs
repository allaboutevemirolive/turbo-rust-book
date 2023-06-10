use std::fs;

fn main() {
    for i in 1..=10 {
        let folder_name = i.to_string();
        fs::create_dir(&folder_name).unwrap_or_else(|err| {
            panic!("Failed to create folder {}: {}", folder_name, err);
        });
    }
}
