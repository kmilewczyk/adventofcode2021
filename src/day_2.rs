use anyhow::{ Result, anyhow };

pub mod cli {
    use crate::day_2::run_dive;
    use crate::core::file::read_lines;
    use crate::command_line::get_input_path;
    use crate::command_line::expect_submatches;
    use crate::command_line::ChallengeSolutionArgs;
    use anyhow::Result;

    const DIVE_SUBCOMMAND: &str = "2_1";

    pub struct SonarDive { }

    impl ChallengeSolutionArgs for SonarDive {
        fn get_subcommand(&self) -> &'static str {
            DIVE_SUBCOMMAND
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> Result<String> { 
            let submatches = expect_submatches(matches, DIVE_SUBCOMMAND);
            let input_path = get_input_path(submatches)?;
            let input = read_lines(input_path)?;
        
            run_dive(input).map(|r| r.to_string())
        }
    }
}

pub fn run_dive<S: AsRef<str>>(input: impl IntoIterator<Item = std::io::Result<S>>) -> Result<isize> {
    let mut depth = 0;
    let mut horizontal = 0;

    for result in input {
        let line = result?;
        let mut splits = line.as_ref().split(' ');
        let command = splits.next().ok_or(anyhow!("The line is empty"))?;
        let value = splits.next().ok_or(anyhow!("There is no second argument"))?.parse::<isize>()?;

        match command {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            command => return Err(anyhow!("Unknown command '{}'", command))
        }
    }

    Ok(depth * horizontal)
}


#[cfg(test)]
mod tests {
    use crate::day_2::run_dive;

    #[test]
    fn it_passes_dive_example_from_description() {
        const INPUT: &str = "forward 5\n\
        down 5\n\
        forward 8\n\
        up 3\n\
        down 8\n\
        forward 2";

        let result = run_dive(INPUT.split('\n').map(|e| Ok(e)));

        match result {
            Ok(val) => assert_eq!(val, 150),
            Err(e) => panic!("{}", e),
        }

    }
}