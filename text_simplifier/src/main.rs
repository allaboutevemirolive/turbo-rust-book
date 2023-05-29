use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

fn main() {
    let input_file    = "input.txt";
    let output_file   = "7.1. Packages and Crates.md";
    let output_file = output_file.replace(" ", "_").to_lowercase();

    let mut new_text = String::new();

    let file = File::open(input_file).expect("Unable to open input file");
    let mut buf_reader = BufReader::new(file);
    let mut data = String::new();
    buf_reader.read_to_string(&mut data).expect("Unable to read input file");

    data = data.replace(". ", ".\n");

    let mut data_chars = data.chars().peekable();
    while let Some(c) = data_chars.next() {
        match c {
            '{' => new_text.push('{'),
            '}' => new_text.push('}'),
            '.' => {
                new_text.push('.');
                if let Some(&next_char) = data_chars.peek() {
                    if next_char != 'r' && !next_char.is_digit(10) {
                        new_text.push('\n');
                    }
                }
            }
            _ => new_text.push(c),
        }
    }

    let output_file = File::create(output_file).expect("Unable to create output file");
    let mut buf_writer = BufWriter::new(output_file);
    buf_writer
        .write_all(new_text.as_bytes())
        .expect("Unable to write to output file");
}