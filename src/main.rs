use std::env;
use std::time::Instant;

use program::Program;

mod program;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a file name\nExampe: cargo run program.txt");
    }
    let now = Instant::now();


    for _ in 0..100 {
        let file_contents = match std::fs::read_to_string(&args[1]) {
            Ok(v) => v,
            Err(e) => panic!("Could not open file: {:?}", e),
        };

        let (width, height) = find_longest_line_and_count_lines(&file_contents);
        let mut program = Program::new(file_contents, width, height);
        program.run();
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn find_longest_line_and_count_lines(file_contents: &str) -> (i32, i32) {
    let mut max_length = 0;
    let mut num_lines = 0;

    for line in file_contents.lines() {
        num_lines += 1;
        let line_length = line.len() as i32;
        if line_length > max_length {
            max_length = line_length;
        }
    }
    
    (max_length, num_lines)
}
