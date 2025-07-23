use crate::VariousCounts;
use crate::TotalVariousCounts;
use crate::counter::{displaying_count_info, getting_data};
use crate::total_counter::total_count_final;
use std::fs::File;
use std::io::stdin;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::path::Path;

pub fn reading_file(arg: &str) -> BufReader<File> {
    let file = File::open(arg).expect("failed to open file");
    let reader = BufReader::new(file);
    reader
}

pub fn input_reading(arg_one: &str) {
    let input = stdin();
    let mut reader = BufReader::new(input.lock());

    match reader.fill_buf() {
        Ok(buf) if buf.is_empty() => {
            eprintln!("no input provided, please provide file path or pass input data via pipe");
            exit(1);
        }
        Ok(_) => {
            // instance of struct variouscounts for storing counts of individual file for each file.
            let mut counts = VariousCounts {
                line_count: 0,
                word_count: 0,
                character_count: 0,
                byte_count: 0,
            };

            getting_data(reader, &mut counts);

            println!("\n----------------------------");
            println!("From input data: ");
            println!("----------------------------\n");

            displaying_count_info(arg_one, &mut counts);
        }
        Err(e) => {
            eprintln!("error reading input, {}", e);
            exit(1);
        }
    }
}

pub fn multiple_file_reading(arguments: &Vec<String>, total_counts: &mut TotalVariousCounts) {
    for i in 2..arguments.len() {
        let file_path = Path::new(&arguments[i]);
        let file_name = file_path.file_name();

        match file_name {
            Some(f) => {
                println!("\n----------------------------");
                println!("{:<7} {}", "File", f.to_string_lossy());
                println!("----------------------------\n");
            }
            None => {
                println!("\n----------------------------");
                println!("{:<4} {}", "File No", &arguments[i]);
                println!("----------------------------\n");
            }
        };

        // calling reading_file function to open and read file.
        let reader = reading_file(&arguments[i]);

        // instance of struct variouscounts for storing counts of individual file for each file.
        let mut counts = VariousCounts {
            line_count: 0,
            word_count: 0,
            character_count: 0,
            byte_count: 0,
        };

        getting_data(reader, &mut counts);

        displaying_count_info(&arguments[1], &mut counts);

        total_count_final(total_counts, &mut counts);
    }
}
