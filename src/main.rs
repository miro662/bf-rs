mod bf;

use bf::Brainfuck;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    // Get file to parse
    let mut args = env::args();
    if args.len() < 2 {
        println!("usage: bf-rs file");
    }
    else {
        // Create new Brainfuck context
        let mut brainfuck = Brainfuck::new();
        
        // Load file to string
        let mut program = String::new();

        let mut file = File::open(args.nth(1).unwrap())
            .expect("Cannot load file!");

        file.read_to_string(&mut program)
            .expect("Cannot load program!");

        println!("{}", program);
        // Call simple code
        brainfuck.call(&program);
        println!("");
    }
}
