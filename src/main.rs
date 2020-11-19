use crate::program::Program;

mod code;
mod parse;
mod program;

fn main() {
    let args = std::env::args();
    assert_eq!(2, args.len());
    let src_file = args.last().unwrap();
    let mut program = Program::from_file(src_file);
    std::process::exit(program.run());
}

