pub trait ChallengeSolutionArgs {
    // Returns the name for the subcommand, by which the args can be indentified back
    fn add_subcommand(&self, app: &mut clap::App) -> &'static str;
    fn run(&mut self, matches: &clap::ArgMatches);
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

trait ClapAppExt {
    fn aoc_solution(self, solution_args: Box<dyn ChallengeSolutionArgs>, resolver: &mut ClapSubcommandResolver) -> Self;
}

impl ClapAppExt for clap::App<'static> {
    fn aoc_solution(mut self, solution_args: Box<dyn ChallengeSolutionArgs>, resolver: &mut ClapSubcommandResolver) -> Self { 
        let subcommand = solution_args.add_subcommand(&mut self);
        resolver.add_subcommand(subcommand, solution_args);
        self
    }
}
