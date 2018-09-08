pub mod assembler;

const MEMORY_SIZE: usize = 256;

pub struct Emulator {
    pub memory: [u8; MEMORY_SIZE],
    pub pc: u8,
    pub sp: u8,
    pub running: bool,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            memory: [0; MEMORY_SIZE],
            pc: 0,
            sp: (MEMORY_SIZE - 1) as u8,
            running: true,
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.next();
        }
    }

    pub fn read(&self, pos: usize) -> u8 {
        self.memory[pos]
    }

    pub fn write(&mut self, pos: usize, data: u8) {
        self.memory[pos] = data;
    }

    fn push(&mut self, data: u8) {
        let pos = self.sp;
        self.write(pos as usize, data);
        self.sp = pos - 1;
    }

    fn pop(&mut self) -> u8 {
        self.sp += 1;
        let data = self.read(self.sp as usize);
        data
    }

    pub fn step(&mut self) {
        if self.running {
            self.next()
        }
    }

    fn next(&mut self) {
        let instruction = self.read(self.pc as usize);

        match instruction {
            0x00 => {
                self.running = false;
            }
            0x01 => {
                self.pc += 1;
                let data = self.memory[self.pc as usize];
                self.push(data);
            }
            0x02 => {
                self.pop();
            }
            0x03 => {
                let a = self.pop();
                let b = self.pop();
                let res = a + b;
                self.push(res);
            }
            _ => {}
        }

        self.pc += 1;
    }
}
