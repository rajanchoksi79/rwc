use std::process::exit;
use crate::{TotalVariousCounts};
use crate::total_counter::displaying_total_count_info;
use crate::reader::{input_reading, multiple_file_reading};

pub fn arg_parsing(arguments:&Vec<String>) {
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
            println!("'-w' or '--words' : for word count");
            println!("'-l' or '--lines' : for line count");
            println!("'-c' or '--characters' : for character count");
            println!("'-b' or '--bytes' : for bytes count");
            println!("'-a' or '--all' : for counts of all the data");
        } else if (arguments[1] != "-l" || arguments[1] != "--lines")
            && (arguments[1] != "-w" || arguments[1] != "--words") 
            && (arguments[1] != "-c" || arguments[1] != "--characters")
            && (arguments[1] != "-b" || arguments[1] != "--bytes")
            && (arguments[1] != "-a" || arguments[1] != "--all")
        {
            println!("please provide valid flag, use '-help' to know more");
        } else {
            input_reading(&arguments[1]);            
        }
    }
    // with three arguments provided, we run our program
    else {
        if arguments[1] == "-help" {
            println!("'-w' or '--words' : for word count");
            println!("'-l' or '--lines' : for line count");
            println!("'-c' or '--characters' : for character count");
            println!("'-b' or '--bytes' : for bytes count");
            println!("'-a' or '--all' : for counts of all the data");
        } else if (arguments[1] != "-l" || arguments[1] != "--lines")
            && (arguments[1] != "-w" || arguments[1] != "--words") 
            && (arguments[1] != "-c" || arguments[1] != "--characters")
            && (arguments[1] != "-b" || arguments[1] != "--bytes")
            && (arguments[1] != "-a" || arguments[1] != "--all")
        {
            println!("please provide valid flag, use '-help' to know more");
        } else {
            // for loop to run over multiple file, if provided.
            multiple_file_reading(&arguments, &mut total_counts);
            displaying_total_count_info(&arguments, &mut total_counts);
        }
    }
}

