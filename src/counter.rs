use std::io::BufRead;
use std::process::exit;
use crate::VariousCounts;

pub fn getting_data<R: BufRead>(reader: R, counts: &mut VariousCounts) {
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


pub fn displaying_count_info(arg_one: &str, counts: &mut VariousCounts) {
    if arg_one == "-l" || arg_one == "--lines" {
        println!("{:<15} {}", "Lines", counts.line_count);
    } else if arg_one == "-w" || arg_one == "--words" {
        println!("{:<15} {}", "Words", counts.word_count);
    } else if arg_one == "-c" || arg_one == "--characters" {
        println!("{:<15} {}", "Characters", counts.character_count);
    } else if arg_one == "-b" || arg_one == "--bytes" {
        println!("{:<15} {}", "Bytes", counts.byte_count);
    } else if arg_one == "-a" || arg_one == "--all" {
        println!("{:<15} {}", "Lines", counts.line_count);
        println!("{:<15} {}", "Words", counts.word_count);
        println!("{:<15} {}", "Characters", counts.character_count);
        println!("{:<15} {}", "Bytes", counts.byte_count);
    }

    println!("\n----------------------------");
}
