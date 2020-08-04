use crate::algorithm::PageReplacementAlgorithm;
use clap::{App, Arg, ArgMatches};

/// Usamos la librería clap para formar una interfaz de usuario en consola simple y con poco código
/// Esta función regresa la instancia de una "aplicación" de clap con toda la configuración incluída
pub fn get_app() -> clap::App<'static, 'static> {
    App::new("Memory Admin Simulator")
        .version("1.0")
        .author("Equipo 7 de Sistemas Operativos")
        // El primer argumento es el algoritmo y hay tres opciones (declaradas en algorithm.rs)
        .arg(
            Arg::with_name("algorithm")
                .required(true)
                .possible_values(&[
                    PageReplacementAlgorithm::FIFO.as_str(),
                    PageReplacementAlgorithm::LRU.as_str(),
                    PageReplacementAlgorithm::Random.as_str(),
                ])
                .help("Sets the algorithm to choose which page gets replaced in memory")
                .takes_value(true)
                .index(1),
        )
        // El segundo es el nombre del archivo por abrir
        .arg(
            Arg::with_name("file")
                .required(true)
                .help("Path to the file with the list of instructions to execute")
                .takes_value(true)
                .empty_values(false)
                .index(2),
        )
        // Los siguientes son opcionales:
        // El tercer es el tamaño de la página en bytes
        .arg(
            Arg::with_name(SizeArgument::Page.as_str())
                .short("p")
                .long("page-size")
                .help("Sets the page size in bytes, defaults to 16 bytes")
                .takes_value(true),
        )
        // El cuarto es el tamaño de la memoria real en bytes
        .arg(
            Arg::with_name(SizeArgument::RealMemory.as_str())
                .short("r")
                .long("real-memory")
                .help("Sets the size of the real memory in bytes, defaults to 2048 bytes")
                .takes_value(true),
        )
        // El quinto es el tamaño del espacio swap en bytes
        .arg(
            Arg::with_name(SizeArgument::SwapSpace.as_str())
                .short("v")
                .long("swap-space")
                .help("Sets the size of the swap space in bytes, defaults to 4096 bytes")
                .takes_value(true),
        )
}

/// Esta función recibe una referencia a un objeto de coincidencias que genera clap y
/// regresa el nombre del archivo que se incluyó
pub fn get_filename<'a>(matches: &'a ArgMatches) -> &'a str {
    matches
        .args
        .get("file")
        .unwrap()
        .vals
        .first()
        .unwrap()
        .to_str()
        .unwrap()
}

/// Usamos este enum para diferenciar entre los argumentos de tamaños
pub enum SizeArgument {
    Page,
    RealMemory,
    SwapSpace,
}

impl SizeArgument {
    /// Esta función se le aplica a una variante del enum y regresa su representación en string
    fn as_str(&self) -> &'static str {
        match self {
            SizeArgument::Page => "page size",
            SizeArgument::RealMemory => "real memory size",
            SizeArgument::SwapSpace => "swap space size",
        }
    }

    /// Esta función se le aplica a una variante del enum y regresa el tamaño por defecto
    fn default(&self) -> usize {
        match self {
            SizeArgument::Page => 16,
            SizeArgument::RealMemory => 2048,
            SizeArgument::SwapSpace => 4096,
        }
    }
}

/// A esta función se le pasa una referencia al objeto de coincidencias de clap y qué tipo de
/// argumento se busca, y si es posible parsear el argumento regresa el tamaño, en otro caso
/// regresa el número por defecto del argumento
pub fn get_size(matches: &ArgMatches, arg: SizeArgument) -> usize {
    if let Some(matched_args) = matches.args.get(arg.as_str()) {
        if let Some(string) = matched_args.vals.first() {
            if let Ok(size) = string.to_str().unwrap().parse::<usize>() {
                return size;
            }
        }
    }
    arg.default()
}
