use crate::algorithm::PageReplacementAlgorithm;
use clap::{App, Arg, ArgMatches};

pub fn get_app() -> clap::App<'static, 'static> {
    App::new("Memory Admin Simulator")
        .version("1.0")
        .author("Mario Jiménez <mario.emilio.j@gmail.com")
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
        .arg(
            Arg::with_name("file")
                .required(true)
                .help("Path to the file with the list of instructions to execute")
                .takes_value(true)
                .empty_values(false)
                .index(2),
        )
        .arg(
            Arg::with_name(SizeArgument::Page.as_str())
                .short("p")
                .long("page-size")
                .help("Sets the page size in bytes, defaults to 16 bytes")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(SizeArgument::RealMem.as_str())
                .short("r")
                .long("real-mem")
                .help("Sets the size of the real memory in bytes, defaults to 2048 bytes")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(SizeArgument::VirtualMem.as_str())
                .short("v")
                .long("virtual-mem")
                .help("Sets the size of the virtual memory in bytes, defaults to 4096 bytes")
                .takes_value(true),
        )
}

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

pub enum SizeArgument {
    Page,
    RealMem,
    VirtualMem,
}

impl SizeArgument {
    fn as_str(&self) -> &'static str {
        match self {
            SizeArgument::Page => "page size",
            SizeArgument::RealMem => "real memory size",
            SizeArgument::VirtualMem => "virtual memory size",
        }
    }

    fn default(&self) -> usize {
        match self {
            SizeArgument::Page => 16,
            SizeArgument::RealMem => 2048,
            SizeArgument::VirtualMem => 4096,
        }
    }
}

pub fn get_size(matches: &ArgMatches, arg: SizeArgument) -> usize {
    match matches.args.get(arg.as_str()) {
        Some(matched_args) => match matched_args.vals.first() {
            Some(string) => match string.to_str().unwrap().parse() {
                Ok(size) => size,
                Err(_) => {
                    println!(
                        "Couldn't parse argument {} as a number, using the default ({})",
                        arg.as_str(),
                        arg.default()
                    );
                    arg.default()
                }
            },
            None => arg.default(),
        },
        None => arg.default(),
    }
}
