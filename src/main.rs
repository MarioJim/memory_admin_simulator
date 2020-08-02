use std::convert::TryFrom;
use std::fs;

mod algorithm;
mod cli;
mod instruction;
mod process;
mod system;
mod time;
mod util;

use algorithm::PageReplacementAlgorithm;
use instruction::Instruction;
use system::System;

fn main() {
    let matches = cli::get_app().get_matches();
    let algorithm = PageReplacementAlgorithm::from_matches(&matches);
    let filename = cli::get_filename(&matches);
    let file = fs::read_to_string(filename)
        .expect(format!("No se encontró el archivo {}", filename).as_ref());
    let page_size = cli::get_size(&matches, cli::SizeArgument::Page);
    let real_memory_size = cli::get_size(&matches, cli::SizeArgument::RealMemory);
    let swap_space_size = cli::get_size(&matches, cli::SizeArgument::SwapSpace);

    let mut system = System::new(algorithm, page_size, real_memory_size, swap_space_size);

    file.lines()
        .map(|line| Instruction::try_from(line))
        .for_each(|maybe_ins| match maybe_ins {
            Ok(ins) => {
                println!("{}", ins);
                system.process_instruction(&ins);
            }
            Err((ins, error)) => {
                println!("Error al analizar instrucción \"{}\"", ins);
                println!("{}", error);
                println!();
            }
        });
}
