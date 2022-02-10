use crate::command_line::ChallengeSolutionArgs;

const SONAR_SWEEP_DEPTH_SUBCOMMAND: &str = "1_1";

pub struct SonarSweepDepth {
}

impl ChallengeSolutionArgs for SonarSweepDepth {
    fn add_subcommand<'a>(&self, mut app: clap::App<'a>) -> (&'static str, clap::App<'a>) {
        app = app.subcommand(clap::App::new(SONAR_SWEEP_DEPTH_SUBCOMMAND)
            .arg(clap::Arg::new("input").short('i').takes_value(true).required(true))
        );

        (SONAR_SWEEP_DEPTH_SUBCOMMAND, app)
    }

    fn run(&mut self, matches: &clap::ArgMatches) -> Result<String, String> { 
        use std::io::BufRead;

        let input = {
            let submatches = matches.subcommand_matches(SONAR_SWEEP_DEPTH_SUBCOMMAND)
                .expect(format!("Subcommand {} was not invoked", SONAR_SWEEP_DEPTH_SUBCOMMAND).as_str());
            
            submatches.value_of("input").ok_or("No input file was given")?
        };

        let reader = {
            let file = std::fs::File::open(input).map_err(|err| format!("Couldn't open file '{}'. Reason: {}", input, err))?;
            std::io::BufReader::new(file)
        };

        let mut previous_value_option: Option<isize> = None;
        let mut increased_counter: isize = 0;

        for result in reader.lines() {
            let line = result.map_err(|err| format!("Error while reading line of file: {}", err))?;
            let value: isize = line.parse::<isize>().map_err(|err| format!("Can't parse line '{}' to isize. {}", line, err))?;

            if let Some(previous_value) = previous_value_option {
                // If this isn't the first element
                if value > previous_value {
                    increased_counter += 1;
                }
                
            }

            previous_value_option = Some(value);
        }

        Ok(format!("Answer: {}", increased_counter))
    }
}
