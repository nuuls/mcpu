extern crate mcpu;

fn main() {
    let tokens = mcpu::assembler::tokenize(
        "PUSH 0x05;JP;DW A 0x1;DW B 0x2;PUSH A;LOAD;PUSH B;LOAD;ADD;HALT;",
    ).unwrap();
    let mem = mcpu::assembler::parse(&tokens).unwrap();
    let mut emu = mcpu::Emulator::new();

    println!("{:?}", mem);

    emu.load(&mem);
    emu.reset();
    emu.run();

    for byte in emu.memory.iter() {
        print!("{} ", byte);
    }
}
