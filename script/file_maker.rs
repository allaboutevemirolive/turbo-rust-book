
// Only file maker

use std::fs;

fn main() {
    for i in 1..=10 {
        let file_name = format!("file_{}.md", i);
        fs::File::create(&file_name).unwrap_or_else(|err| {
            panic!("Failed to create file {}: {}", file_name, err);
        });
    }
}



// file with data maker

use std::fs;
use std::io::Write;

fn main() {
    for i in 1..=10 {
        let file_name = format!("file{}.md", i);
        let content = format!("This is file {}", i);

        fs::write(&file_name, &content).unwrap_or_else(|err| {
            panic!("Failed to create file {}: {}", file_name, err);
        });
    }
}
