use clap::ArgMatches;

#[derive(Debug)]
pub enum PageReplacementAlgorithm {
    FIFO,
    LRU,
}

impl PageReplacementAlgorithm {
    pub fn as_str(&self) -> &'static str {
        match self {
            PageReplacementAlgorithm::FIFO => "fifo",
            PageReplacementAlgorithm::LRU => "lru",
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
            _ => panic!("No algorithm exists with that name"),
        }
    }
}
