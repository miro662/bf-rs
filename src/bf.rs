//! Brainfuck interpreter

use std::io;
use std::io::Read;

/// Structure which describes Brainfuck context
pub struct Brainfuck {
    /// Memory
    pub memory:[u8; 30000],
    /// Pointer to memory
    pub pointer: usize
}

impl Brainfuck {
    /// Creates new Brainfuck context
    pub fn new() -> Brainfuck {
        Brainfuck {
            memory: [0; 30000],
            pointer: 0
        }
    }

    /// Calls given Brainfuck code
    pub fn call(&mut self, code:&str) {
        // Transform string to array of letters (commands)
        let commands:Vec<char> = code.chars().collect();

        let mut current_command:usize = 0;

        let mut ret_stack:Vec<usize> = vec![];
        
        while current_command < commands.len() {
            let command = commands[current_command];
            let mut move_cc = true;
            match command {
                // Add 1 to pointed memory cell
                '+' => self.memory[self.pointer] = if self.memory[self.pointer] == 255 {
                        0
                    } else {
                        self.memory[self.pointer] + 1
                    },
                // Subtract 1 to pointed memory cell
                '-' => self.memory[self.pointer] = if self.memory[self.pointer] == 0 {
                        255
                    } else {
                        self.memory[self.pointer] - 1
                    },
                // Move memory cell by 1 right
                '>' => self.pointer = if self.pointer == 29999 {
                        0
                    } else {
                        self.pointer + 1
                    },
                // Move memory cell by 1 left
                '<' => self.pointer = if self.pointer == 0 {
                        29999
                    } else {
                        self.pointer - 1
                    },
                // Print ASCII character identified by current memory cell
                '.' => print!("{}", self.memory[self.pointer] as char),
                // Read ASCII character identified by current memory cell
                ',' => {
                    let mut buf:[u8; 1] = [0; 1];
                    if let Err(_) = io::stdin().read(&mut buf) {
                        panic!("Input error")
                    }
                    self.memory[self.pointer] = buf[0];
                },
                // Beginning of loop
                '[' => {
                    // Push beginning to return stack
                    ret_stack.push(current_command);
                }
                // End of loop
                ']' => {
                    // Go to beginning of loop
                    current_command = ret_stack.pop().unwrap();
                    move_cc = false;
                }
                _ => ()
            }
            // Increment current command
            if move_cc {
                current_command += 1;
            }
        }
    }
}
