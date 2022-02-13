use anyhow::{ Result, anyhow };


pub mod cli {
    use crate::command_line::read_input_from_matches;
    use crate::day_2::run_aimed_dive;
    use crate::day_2::run_dive;
    use crate::command_line::ChallengeSolutionArgs;
    use anyhow::Result;

    const DIVE_SUBCOMMAND: &str = "2_1";
    const AIMED_DIVE_SUBCOMMAND: &str = "2_2";

    pub struct SonarDive { }

    impl ChallengeSolutionArgs for SonarDive {
        fn get_subcommand(&self) -> &'static str {
            DIVE_SUBCOMMAND
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> Result<String> { 
            let input = read_input_from_matches(self, matches)?;
        
            run_dive(input).map(|r| r.to_string())
        }
    }

    pub struct SonarAimedDive { }

    impl ChallengeSolutionArgs for SonarAimedDive {
        fn get_subcommand(&self) -> &'static str {
            AIMED_DIVE_SUBCOMMAND
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> Result<String> { 
            let input = read_input_from_matches(self, matches)?;
        
            run_aimed_dive(input).map(|r| r.to_string())
        }
    }

}

fn parse_result<'a>(line: &'a str) -> Result<(&'a str, isize)> {
    let mut splits = line.split(' ');
    let command = splits.next().ok_or(anyhow!("The line is empty"))?;
    let value = splits.next().ok_or(anyhow!("There is no second argument"))?.parse::<isize>()?;
    
    Ok((command, value))
}

pub fn run_dive<S: AsRef<str>>(input: impl IntoIterator<Item = std::io::Result<S>>) -> Result<isize> {
    let mut depth = 0;
    let mut horizontal = 0;

    for result in input {
        let line = result?;

        let (command, value) = parse_result(line.as_ref())?;

        match command {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            command => return Err(anyhow!("Unknown command '{}'", command))
        }
    }

    Ok(depth * horizontal)
}

pub fn run_aimed_dive<S: AsRef<str>>(input: impl IntoIterator<Item = std::io::Result<S>>) -> Result<isize> {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for result in input {
        let line = result?;

        let (command, value) = parse_result(line.as_ref())?;

        match command {
            "forward" => { 
                horizontal += value;
                depth += value * aim;
            },
            "down" => aim += value,
            "up" => aim -= value,
            command => return Err(anyhow!("Unknown command '{}'", command))
        }
    }

    Ok(depth * horizontal)
}


#[cfg(test)]
mod tests {
    use crate::day_2::run_aimed_dive;
    use crate::day_2::run_dive;

    const INPUT: &str = "\
        forward 5\n\
        down 5\n\
        forward 8\n\
        up 3\n\
        down 8\n\
        forward 2";

    #[test]
    fn it_passes_dive_example_from_description() {
        let result = run_dive(INPUT.split('\n').map(|e| Ok(e)));

        match result {
            Ok(val) => assert_eq!(val, 150),
            Err(e) => panic!("{}", e),
        }

    }

    #[test]
    fn it_passes_aimed_dive_example_from_description() {
        let result = run_aimed_dive(INPUT.split('\n').map(|e| Ok(e)));

        match result {
            Ok(val) => assert_eq!(val, 900),
            Err(e) => panic!("{}", e),
        }
    }
}