use clap::ArgMatches;

/// Usamos este enum para seleccionar qué algoritmo usará el sistema
#[derive(Debug)]
pub enum PageReplacementAlgorithm {
    FIFO,
    LRU,
    Random,
}

impl PageReplacementAlgorithm {
    /// Esta función se aplica a una variante del enum y regresa un string
    pub fn as_str(&self) -> &'static str {
        match self {
            PageReplacementAlgorithm::FIFO => "fifo",
            PageReplacementAlgorithm::LRU => "lru",
            PageReplacementAlgorithm::Random => "rand",
        }
    }

    /// Esta función recibe una referencia a un objeto con las coincidencias de los argumentos
    /// pasados al programa y regresa qué tipo de algoritmo se eligió
    pub fn from_matches(matches: &ArgMatches) -> Self {
        match matches
            .args
            .get("algorithm")
            .unwrap()
            .vals
            .first()
            .unwrap()
            .to_str()
            .unwrap()
        {
            "fifo" => PageReplacementAlgorithm::FIFO,
            "lru" => PageReplacementAlgorithm::LRU,
            "rand" => PageReplacementAlgorithm::Random,
            _ => panic!("Un algoritmo con ese nombre no se ha implementado"),
        }
    }
}
