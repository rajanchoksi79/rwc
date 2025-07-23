use std::path::Path;
use crate::reader::reading_file;
use crate::{TotalVariousCounts, VariousCounts};
use crate::total_counter::total_count_final;
use crate::counter::{getting_data, displaying_count_info};


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
