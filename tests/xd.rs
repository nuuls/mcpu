extern crate mcpu;

use std::fs::File;
use std::io::Write;

fn dump_stack(emulator: &mcpu::Emulator) {
    let mut file = File::create("stack.txt").unwrap();

    for entry in emulator.memory.iter() {
        writeln!(&mut file, "{:X}", entry).unwrap();
    }
}

#[test]
fn it_adds() {
    let mut emulator = mcpu::Emulator::new();
    emulator.write(0, 0x01);
    emulator.write(1, 0x01);
    emulator.write(2, 0x01);
    emulator.write(3, 0x01);
    emulator.write(4, 0x03);

    emulator.run();

    assert_eq!(emulator.read((emulator.sp + 1) as usize), 2);
}

#[test]
fn it_parses() {
    match mcpu::assembler::parse("PUSH 0x8\nADD\n") {
        Ok(tokens) => {
            println!("{:?}", tokens);
            assert_eq!(tokens.len(), 5)
        }
        Err(e) => panic!("{}", e),
    }
}

#[test]
fn it_assembles() {
    match mcpu::assembler::parse("PUSH 0x1\nPUSH 0x1\nADD\n") {
        Ok(tokens) => {
            println!("{:?}", tokens);
            match mcpu::assembler::assemble(&tokens) {
                Ok(mem) => {
                    println!("{:?}", mem);
                }
                Err(e) => panic!("{}", e),
            }
        }
        Err(e) => panic!("{}", e),
    }
}
