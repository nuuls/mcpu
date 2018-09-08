extern crate mcpu;

fn main() {
    let tokens = mcpu::assembler::tokenize(
        "push 0x05\njp\ndw sum 0x0\ndw i 0xa\npush i\nload\npush sum\nload\nadd\npush sum\nstore\npush 0x1\npush i\nload\nsub\npush i\nstore\npush i\nload\npush 0x05\njp neq\npush sum\nload\n",
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
