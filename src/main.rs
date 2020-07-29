use std::convert::TryFrom;
use std::fs;

mod algorithm;
mod cli;
mod instructions;
mod process;
mod system;

use algorithm::PageReplacementAlgorithm;
use instructions::Instruction;
use system::System;

fn main() {
    let matches = cli::get_app().get_matches();
    let algorithm = PageReplacementAlgorithm::from_matches(&matches);
    let filename = cli::get_filename(&matches);
    let file = fs::read_to_string(filename)
        .expect(format!("No se encontró el archivo {}", filename).as_ref());
    let page_size = cli::get_size(&matches, cli::SizeArgument::Page);
    let mem_size = cli::get_size(&matches, cli::SizeArgument::Swap);
    let swap_size = cli::get_size(&matches, cli::SizeArgument::Memory);

    let mut system = System::new(algorithm, page_size, mem_size, swap_size);

    file.lines()
        .map(|line| Instruction::try_from(line))
        .for_each(|maybe_ins| match maybe_ins {
            Ok(ins) => {
                println!("{}", ins);
                system.process_instruction(&ins);
            }
            Err(e) => println!("Error: {}", e),
        });
}
