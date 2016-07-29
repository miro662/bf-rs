//! Brainfuck interpreter

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
        for command in code.chars() {
            match command {
                '+' => self.memory[self.pointer] = if self.memory[self.pointer] == 255 {
                        0
                    } else {
                        self.memory[self.pointer] + 1
                    },
                '-' => self.memory[self.pointer] = if self.memory[self.pointer] == 0 {
                        255
                    } else {
                        self.memory[self.pointer] - 1
                    },
                _ => ()
            }
        }
    }
}
