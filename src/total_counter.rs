use crate::TotalVariousCounts;
use crate::VariousCounts;

pub fn total_count_final(total_counts: &mut TotalVariousCounts, counts: &mut VariousCounts) {
    total_counts.total_line_count += counts.line_count;
    total_counts.total_character_count += counts.character_count;
    total_counts.total_word_count += counts.word_count;
    total_counts.total_byte_count += counts.byte_count;
}

pub fn displaying_total_count_info(args: &Vec<String>, total_counts: &mut TotalVariousCounts) {
    println!("\n----------------------------");
    println!("{} {:10}", "Total files", args.len() - 2);
    println!("----------------------------\n");

    if args[1] == "-l" || args[1] == "--lines" {
        println!("{:<20} {}", "Total lines", total_counts.total_line_count);
    } else if args[1] == "-w" || args[1] == "--words" {
        println!("{:<20} {}", "Total words", total_counts.total_word_count);
    } else if args[1] == "-c" || args[1] == "--characters" {
        println!("{:<20} {}", "Total characters", total_counts.total_character_count);
    } else if args[1] == "-b" || args[1] == "--bytes" {
        println!("{:<20} {}", "Total bytes", total_counts.total_byte_count);
    } else if args[1] == "-a" || args[1] == "--all" {
        println!("{:<20} {}", "Total lines", total_counts.total_line_count);
        println!("{:<20} {}", "Total words", total_counts.total_word_count);
        println!("{:<20} {}", "Total characters", total_counts.total_character_count);
        println!("{:<20} {}", "Total bytes", total_counts.total_byte_count);
    }

    println!("\n----------------------------");
}
