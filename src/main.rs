use crate::day_4::cli::GiantSquid;
use crate::day_3::cli::LifeSupportRating;
use crate::day_3::cli::BinaryDiagnostic;
use crate::day_2::cli::SonarAimedDive;
use crate::day_2::cli::SonarDive;
use crate::day_1::cli::SonarSlidingWindow;
use crate::day_1::cli::SonarSweepDepth;

const PROGRAM_NAME: &str = "Advent of Code 2021 Solutions";
const VERSION: &str = "0.1.0";
const AUTHOR: &str = "Karol Milewczyk";
const ABOUT: &str = "The answer programs for Advent of Code 2021.";
const DESCRIPTION: &str = "\
This program consists of ready solutions to Advent of Code 2021, a programming challenge
hosted on 'adventofcode.com'.
";

mod command_line;
mod core;
mod day_1;
mod day_2;
mod day_3;
mod day_4;


fn get_cli_matches(resolver: &mut command_line::ClapSubcommandResolver) -> clap::ArgMatches {
    use command_line::ClapAppExt;

    clap::App::new(PROGRAM_NAME)
        .author(AUTHOR)
        .version(VERSION)
        .about(ABOUT)
        .after_help(DESCRIPTION)
        .aoc_solution(Box::new(SonarSweepDepth {}), resolver)
        .aoc_solution(Box::new(SonarSlidingWindow {}), resolver)
        .aoc_solution(Box::new(SonarDive {}), resolver)
        .aoc_solution(Box::new(SonarAimedDive {}), resolver)
        .aoc_solution(Box::new(BinaryDiagnostic {}), resolver)
        .aoc_solution(Box::new(LifeSupportRating {}), resolver)
        .aoc_solution(Box::new(GiantSquid {}), resolver)
        .get_matches()
}


fn main() {
    env_logger::init();

    let mut resolver = command_line::ClapSubcommandResolver::new();
    let m = get_cli_matches(&mut resolver);

    let output = match resolver.resolve(&m) {
        Ok(solution_args) => { solution_args.run(&m) },
        Err(err) => Err(err)
    };

    match output {
        Ok(result) => { println!("{}", result) },
        Err(err) => { log::error!("{}", err) }
    };
}
