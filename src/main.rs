mod total_counter;
mod reader;
mod args;
mod counter;

use crate::total_counter::displaying_total_count_info;
use crate::reader::input_reading;
use crate::args::multiple_file_reading;

use std::env::args;
use std::process::exit;

struct VariousCounts {
    line_count: usize,
    word_count: usize,
    character_count: usize,
    byte_count: usize,
}

struct TotalVariousCounts {
    total_line_count: usize,
    total_word_count: usize,
    total_character_count: usize,
    total_byte_count: usize,
}


fn main() {
    // collecting argument as a vector.
    let arguments: Vec<String> = args().collect();

    // instance of struct totalvariouscounts for storing total counts across all the files.
    let mut total_counts = TotalVariousCounts {
        total_line_count: 0,
        total_word_count: 0,
        total_character_count: 0,
        total_byte_count: 0,
    };

    if arguments.len() == 1 {
        println!("\n---------------------------------------------");
        println!(
            "Welcome to 'rwc' the tool that gives you counts of lines, words, character and bytes of any text file/files\nplease provide input or file path with valid flag, use '-- -help' to know more about flags"
        );
        println!("-----------------------------------------------\n");

        exit(1);
    } else if arguments.len() == 2 {
        if arguments[1] == "-help" {
            println!("'-- -w' : for word count");
            println!("'-- -l' : for line count");
            println!("'-- -c' : for character count");
            println!("'-- -b' : for bytes count");
            println!("'-- -a' : for counts of all the data");
        } else if arguments[1] != "-l"
            && arguments[1] != "-w"
            && arguments[1] != "-c"
            && arguments[1] != "-b"
            && arguments[1] != "-a"
        {
            println!("please provide valid flag, use '-- -help' to know more");
        } else {
            input_reading(&arguments[1]);            
        }
    }
    // with three arguments provided, we run our program
    else {
        if arguments[1] == "-help" {
            println!("'-- -w' : for word count");
            println!("'-- -l' : for line count");
            println!("'-- -c' : for character count");
            println!("'-- -b' : for bytes count");
            println!("'-- -a' : for counts of all the data");
        } else if arguments[1] != "-l"
            && arguments[1] != "-w"
            && arguments[1] != "-c"
            && arguments[1] != "-b"
            && arguments[1] != "-a"
        {
            println!("please provide valid flag, use '-- -help' to know more");
        } else {
            // for loop to run over multiple file, if provided.
            multiple_file_reading(&arguments, &mut total_counts);
            displaying_total_count_info(&arguments, &mut total_counts);
        }
    }
}
