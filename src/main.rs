use std::env;

mod program;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args.len());
    if args.len() < 2 {
        panic!("Please specify a file name\nExampe: cargo run program.txt");
    }
}
