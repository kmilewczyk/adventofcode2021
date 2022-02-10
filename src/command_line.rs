pub trait ChallengeSolutionArgs {
    // Returns the name for the subcommand, by which the args can be indentified back
    fn add_subcommand<'a>(&self, app: clap::App<'a>) -> (&'static str, clap::App<'a>);
    fn run(&mut self, matches: &clap::ArgMatches) -> Result<String, String>;
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

    pub fn resolve(&mut self, matches: &clap::ArgMatches) -> Result<&mut Box<dyn ChallengeSolutionArgs>, String>{
        match matches.subcommand() {
            Some((subcommand, _)) => self.subcommands.get_mut(subcommand).ok_or(format!("'{}' is not known subcommand.", subcommand)),
            None => Err(String::from("No command was specified."))
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

pub fn get_input(matches: &clap::ArgMatches) -> Result<&str, String> {
    matches.value_of("input").ok_or(format!("No input file was given"))
}

pub fn expect_submatches<'a>(matches: &'a clap::ArgMatches, subcommand: &'static str) -> &'a clap::ArgMatches{
    matches.subcommand_matches(subcommand)
        .expect(format!("Subcommand {} was not invoked", subcommand).as_str())
}