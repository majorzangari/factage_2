use std::env;
use std::fs;

use program::Program;

mod program;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a file name\nExampe: cargo run program.txt");
    }

    let file_contents = match std::fs::read_to_string(&args[1]) {
        Ok(v) => v,
        Err(e) => panic!("Could not open file: {:?}", e),
    };

    let (width, height) = find_longest_line_and_count_lines(&file_contents);

}

fn find_longest_line_and_count_lines(file_contents: &str) -> (usize, usize) {
    let mut max_length = 0;
    let mut num_lines = 0;

    for line in file_contents.lines() {
        num_lines += 1;
        let line_length = line.len();
        if line_length > max_length {
            max_length = line_length;
        }
    }
    
    (max_length, num_lines)
}
