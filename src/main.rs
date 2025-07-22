use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
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

// this function is for opening the file and reading it's contents and then returning it.
fn reading_file(arg: &str) -> BufReader<File> {
    // opening the file at given path
    let file = File::open(arg).expect("failed to open file");

    // reading file with bufreader.
    let reader = BufReader::new(file);

    // returning the reader.
    reader
}

// this fuvntion is for getting various data from each line and calculating and counting of it.
fn getting_data<R: BufRead>(reader: R, counts: &mut VariousCounts) {
    // running for loop on each line and getting results.
    for line in reader.lines() {
        match line {
            Ok(l) => {
                counts.line_count += 1;
                counts.word_count += l.split_whitespace().count();
                counts.character_count += l.chars().count();
                counts.byte_count += l.bytes().count();
            }
            Err(e) => {
                eprintln!("error occured while reading line, {e}");
                exit(1);
            }
        }
    }
}

// this function is for displaying info to user, based on flags provided and it no flag is provided then assiting user to give into about flags.
fn displaying_count_info(arg_one: &str, counts: &mut VariousCounts) {
    // displaying info based on flag provided with if and else if and if not valid provided then else condition ran and give info to user to provide valid flag.
    if arg_one == "-l" {
        println!("Lines : {}", counts.line_count);
    } else if arg_one == "-w" {
        println!("Words : {}", counts.word_count);
    } else if arg_one == "-c" {
        println!("Characters : {}", counts.character_count);
    } else if arg_one == "-b" {
        println!("Bytes : {}", counts.byte_count);
    } else if arg_one == "-a" {
        println!("Lines : {}", counts.line_count);
        println!("Words : {}", counts.word_count);
        println!("Characters : {}", counts.character_count);
        println!("Bytes : {}", counts.byte_count);
    }

    println!("\n----------------------");
}

// we add counts of each item of each file at the end of loop to total count of that item.
fn total_count_final(total_counts: &mut TotalVariousCounts, counts: &mut VariousCounts) {
    total_counts.total_line_count += counts.line_count;
    total_counts.total_character_count += counts.character_count;
    total_counts.total_word_count += counts.word_count;
    total_counts.total_byte_count += counts.byte_count;
}

// this function is for displaying total info at the end to user, based on flags provided.
fn displaying_total_count_info(args: &Vec<String>, total_counts: &mut TotalVariousCounts) {
    println!("\n----------------------");
    println!("Total files : {}", args.len() - 2);
    println!("----------------------\n");

    if args[1] == "-l" {
        println!("Total lines : {}", total_counts.total_line_count);
    } else if args[1] == "-w" {
        println!("Total words : {}", total_counts.total_word_count);
    } else if args[1] == "-c" {
        println!("Total characters : {}", total_counts.total_character_count);
    } else if args[1] == "-b" {
        println!("Total bytes : {}", total_counts.total_byte_count);
    } else if args[1] == "-a" {
        println!("Total lines : {}", total_counts.total_line_count);
        println!("Total words : {}", total_counts.total_word_count);
        println!("Total characters : {}", total_counts.total_character_count);
        println!("Total bytes : {}", total_counts.total_byte_count);
    }

    println!("\n----------------------");
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
            "Welcome to 'rwc' the tool that gives you counts of lines, words, character and bytes of any text file/files\n"
        );
        println!(
            "please provide input or file path with valid flag, use '-- -help' to know more about flags"
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
            let input = stdin();
            let mut reader = BufReader::new(input.lock());

            match reader.fill_buf() {
                Ok(buf) if buf.is_empty() => {
                    eprintln!(
                        "no input provided, please provide file path or pass input data via pipe"
                    );
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

                    println!("\n----------------------");
                    println!("From input data: ");
                    println!("----------------------\n");

                    displaying_count_info(&arguments[1], &mut counts);
                }
                Err(e) => {
                    eprintln!("error reading input, {}", e);
                    exit(1);
                }
            }
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
            for i in 2..arguments.len() {
                // displaying file number as initial info.
                println!("\n----------------------");
                println!("File no. : {}", i - 1);
                println!("----------------------\n");

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

                total_count_final(&mut total_counts, &mut counts);
            }

            displaying_total_count_info(&arguments, &mut total_counts);
        }
    }
}
