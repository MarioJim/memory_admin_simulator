use clap::ArgMatches;

#[derive(Debug)]
pub enum PageReplacementAlgorithm {
    FIFO,
    LRU,
    Random,
}

impl PageReplacementAlgorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            PageReplacementAlgorithm::FIFO => "fifo",
            PageReplacementAlgorithm::LRU => "lru",
            PageReplacementAlgorithm::Random => "rand",
        }
    }

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
