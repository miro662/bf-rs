mod bf;

use bf::Brainfuck;

fn main() {
    // Create new Brainfuck context
    let mut brainfuck = Brainfuck::new();

    // Call simple code
    brainfuck.call("[,.]");
    println!("");
}
