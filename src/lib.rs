pub mod assembler;

const MEMORY_SIZE: usize = 256;
const PC: usize = MEMORY_SIZE - 1;
const SP: usize = MEMORY_SIZE - 2;

pub struct Emulator {
    pub memory: [u8; MEMORY_SIZE],
    pub running: bool,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            memory: [0; MEMORY_SIZE],
            running: true,
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.next();
        }
    }

    pub fn early_halt(&mut self) {
        self.running = false;
    }

    pub fn reset(&mut self) {
        self.write(PC, 0);
        self.write(SP, (MEMORY_SIZE - 3) as u8);
        self.running = true;
    }

    pub fn load(&mut self, program: &[u8]) {
        for i in 0..program.len() {
            self.write(i, program[i]);
        }
    }

    pub fn read(&self, pos: usize) -> u8 {
        self.memory[pos]
    }

    pub fn write(&mut self, pos: usize, data: u8) {
        self.memory[pos] = data;
    }

    fn push(&mut self, data: u8) {
        let pos = self.read(SP);
        self.write(pos as usize, data);
        self.write(SP, pos - 1);
    }

    fn pop(&mut self) -> u8 {
        self.memory[SP] += 1;
        let data = self.read(self.memory[SP] as usize);
        data
    }

    pub fn step(&mut self) {
        if self.running {
            self.next()
        }
    }

    fn next(&mut self) {
        let instruction = self.read(self.memory[PC] as usize);

        match instruction {
            0x00 => {
                self.running = false;
            }
            0x01 => {
                let location = self.pop();
                let data = self.read(location as usize);
                self.push(data);
            }
            0x02 => {
                let location = self.pop();
                let data = self.pop();
                self.write(location as usize, data);
            }
            0x03 => {
                self.memory[PC] += 1;
                let location = self.memory[PC];
                let data = self.read(location as usize);
                self.push(data);
            }
            0x04 => {
                self.pop();
            }
            0x05 => {
                let a = self.pop() as i8;
                let b = self.pop() as i8;
                self.push((a + b) as u8);
            }
            0x06 => {
                let a = self.pop() as i8;
                let b = self.pop() as i8;
                self.push((a - b) as u8);
            }
            0x07 => {
                let a = self.pop();
                let b = self.pop();
                self.push(a & b);
            }
            0x08 => {
                let a = self.pop();
                let b = self.pop();
                self.push(a | b);
            }
            0x09 => {
                let a = self.pop();
                let b = self.pop();
                self.push(a ^ b);
            }
            0x0A => {
                self.memory[PC] += 1;

                let location = self.pop();
                let cond_location = self.read(PC);
                let condition = self.read(cond_location as usize);

                let should_jump = match condition {
                    0x00 => true,
                    0x01...0x06 => {
                        let data = self.pop() as i8;
                        match condition {
                            0x01 => data > 0,
                            0x02 => data < 0,
                            0x03 => data >= 0,
                            0x04 => data <= 0,
                            0x05 => data == 0,
                            0x06 => data != 0,
                            _ => false,
                        }
                    }
                    _ => false,
                };

                if should_jump {
                    self.write(PC, location);
                }
            }
            _ => {}
        }

        self.memory[PC] += 1;
    }
}
