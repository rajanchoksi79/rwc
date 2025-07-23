mod total_counter;
mod reader;
mod args;
mod counter;

use std::env::args;
use crate::args::arg_parsing;

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
    let arguments: Vec<String> = args().collect();
    arg_parsing(&arguments);
}
