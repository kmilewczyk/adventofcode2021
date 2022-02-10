use crate::command_line::expect_submatches;
use crate::command_line::add_input;
use crate::command_line::get_input;
use crate::command_line::ChallengeSolutionArgs;
use crate::core::file::read_lines;

const SONAR_SWEEP_DEPTH_SUBCOMMAND: &str = "1_1";

fn read_value(read_result: std::io::Result<String>) -> Result<isize, String> {
    let line = read_result.map_err(|err| format!("Error while reading line of file: {}", err))?;

    line.parse::<isize>().map_err(|err| format!("Can't parse line '{}' to isize. {}", line, err))
}

pub struct SonarSweepDepth {
}

impl ChallengeSolutionArgs for SonarSweepDepth {
    fn add_subcommand<'a>(&self, app: clap::App<'a>) -> (&'static str, clap::App<'a>) {
        (SONAR_SWEEP_DEPTH_SUBCOMMAND, add_input(app, SONAR_SWEEP_DEPTH_SUBCOMMAND))
    }

    fn run(&mut self, matches: &clap::ArgMatches) -> Result<String, String> { 

        let submatches = expect_submatches(matches, SONAR_SWEEP_DEPTH_SUBCOMMAND);

        let input = get_input(submatches)?;

        let mut previous_value_option: Option<isize> = None;
        let mut increased_counter: isize = 0;


        for result in read_lines(input)? {
            let value = read_value(result)?;

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
