const PROGRAM_NAME: &str = "Advent of Code 2021 Solutions";
const VERSION: &str = "0.1.0";
const AUTHOR: &str = "Karol Milewczyk";
const ABOUT: &str = "The answer programs for Advent of Code 2021.";
const DESCRIPTION: &str = "\
This program consists of ready solutions to Advent of Code 2021, a programming challenge
hosted on 'adventofcode.com'.
";

mod command_line;

pub mod core {
    pub trait ChallengeSolution {
        fn get_description(&self) -> &str;
        fn add_input(&mut self, name: &str, input: String) -> Result<(), &str>;
        fn add_input_file(&mut self, name: &str, file: std::path::Path) -> Result<(), &str>;
        fn run(&mut self) -> Result<&str, &str>;
    }
}

pub mod day_1 {
    struct DayOneSolution {}
}


fn get_cli_matches(resolver: &mut command_line::ClapSubcommandResolver) -> clap::ArgMatches {
    clap::App::new(PROGRAM_NAME)
        .author(AUTHOR)
        .version(VERSION)
        .about(ABOUT)
        .after_help(DESCRIPTION)
        .get_matches()
}


fn main() {
    let mut resolver = command_line::ClapSubcommandResolver::new();
    let m = get_cli_matches(&mut resolver);

    match resolver.resolve(&m) {
        Ok(solution_args) => { solution_args.run(&m) },
        Err(err) => { log::error!("{}", err) }
    }
}
