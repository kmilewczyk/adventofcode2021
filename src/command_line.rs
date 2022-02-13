use crate::core::file::read_lines;
use anyhow::Result;
use anyhow::anyhow;

pub trait ChallengeSolutionArgs {
    // Returns the name for the subcommand, by which the args can be indentified back
    fn run(&mut self, matches: &clap::ArgMatches) -> Result<String>;
    fn get_subcommand(&self) -> &'static str;
}

impl dyn ChallengeSolutionArgs {
    fn add_subcommand<'a>(&self, app: clap::App<'a>) -> (&'static str, clap::App<'a>) {
        (self.get_subcommand(), add_input(app, self.get_subcommand()))
    }

}

pub fn read_input_from_matches(args: &impl ChallengeSolutionArgs, matches: &clap::ArgMatches) -> anyhow::Result<std::io::Lines<std::io::BufReader<std::fs::File>>> {
    use anyhow::Context;

    let submatches = expect_submatches(matches, args.get_subcommand());
    let input_path = get_input_path(submatches)?;

    read_lines(input_path).with_context(|| "Failed to read file")
}

pub struct ClapSubcommandResolver {
    subcommands: std::collections::HashMap<&'static str, Box<dyn ChallengeSolutionArgs>>
}

impl ClapSubcommandResolver {
    pub fn new() -> Self {
        Self {
            subcommands: std::collections::HashMap::new()
        }
    }

    fn add_subcommand(&mut self, subcommand: &'static str, solution_args: Box<dyn ChallengeSolutionArgs>) {
        self.subcommands.insert(subcommand, solution_args);
    }

    pub fn resolve(&mut self, matches: &clap::ArgMatches) -> Result<&mut Box<dyn ChallengeSolutionArgs>>{
        match matches.subcommand() {
            Some((subcommand, _)) => self.subcommands.get_mut(subcommand).ok_or(anyhow!("'{}' is not known subcommand.", subcommand)),
            None => Err(anyhow!("No command was specified."))
        }
    }
}

pub trait ClapAppExt {
    fn aoc_solution(self, solution_args: Box<dyn ChallengeSolutionArgs>, resolver: &mut ClapSubcommandResolver) -> Self;
}

impl ClapAppExt for clap::App<'static> {
    fn aoc_solution(self, solution_args: Box<dyn ChallengeSolutionArgs>, resolver: &mut ClapSubcommandResolver) -> Self { 
        let (subcommand, app) = solution_args.add_subcommand(self);
        resolver.add_subcommand(subcommand, solution_args);
        app
    }
}


pub fn add_input<'a>(app: clap::App<'a>, subcommand: &'static str) -> clap::App<'a> {
    app.subcommand(clap::App::new(subcommand)
        .arg(clap::Arg::new("input").short('i').takes_value(true).required(true))
    )
}

pub fn get_input_path(matches: &clap::ArgMatches) -> Result<&str> {
    matches.value_of("input").ok_or(anyhow!("No input file was given"))
}

pub fn expect_submatches<'a>(matches: &'a clap::ArgMatches, subcommand: &'static str) -> &'a clap::ArgMatches{
    matches.subcommand_matches(subcommand)
        .expect(format!("Subcommand {} was not invoked", subcommand).as_str())
}
