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

/// Punto de entrada del programa
fn main() {
    // clap parsea los argumentos de la consola y los guarda en un objeto de coincidencias
    let matches = cli::get_app().get_matches();
    // Se obtiene el algoritmo
    let algorithm = PageReplacementAlgorithm::from_matches(&matches);
    // Se obtiene el nombre del archivo
    let filename = cli::get_filename(&matches);
    // Se abre el archivo y se lee
    let file = fs::read_to_string(filename)
        .expect(format!("No se encontró el archivo {}", filename).as_ref());
    // Se obtiene el tamaño de página
    let page_size = cli::get_size(&matches, cli::SizeArgument::Page);
    // Se obtiene el tamaño de la memoria real
    let real_memory_size = cli::get_size(&matches, cli::SizeArgument::RealMemory);
    // Se obtiene el tamaño del espacio swap
    let swap_space_size = cli::get_size(&matches, cli::SizeArgument::SwapSpace);
    // Se instancía el sistema pasándole el algoritmo, el tamaño de página, de memoria real y de
    // espacio swap
    let mut system = System::new(algorithm, page_size, real_memory_size, swap_space_size);

    // Por cada línea del archivo
    file.lines()
        // Se intenta convertir la línea en una instrucción
        .map(|line| Instruction::try_from(line))
        // Por cada posible instrucción
        .for_each(|maybe_ins| {
            match maybe_ins {
                // Si la instrucción se pudo parsear se manda a que el sistema la ejecute
                Ok(ins) => {
                    println!("{}", ins);
                    system.process_instruction(&ins);
                }
                // En otro caso se imprime un error
                Err((ins, error)) => {
                    if ins.len() > 0 {
                        println!("{}", ins);
                    }
                    println!("Error al analizar instrucción: {}", error);
                }
            }
            println!();
        });
}
