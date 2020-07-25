use std::convert::TryFrom;
use std::env;
use std::fs;

mod instructions;

use instructions::Instruction;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = fs::read_to_string(filename)
        .expect(format!("No se encontró el archivo {}", filename).as_ref());
    file.lines()
        .map(|line| Instruction::try_from(line))
        .for_each(|maybe_ins| match maybe_ins {
            Ok(ins) => println!("{:?}", ins),
            Err(e) => println!("Error: {}", e),
        });
}
