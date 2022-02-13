use std::collections::VecDeque;
use anyhow::Result;

pub mod cli {
    use crate::command_line::read_input_from_matches;
    use crate::command_line::ChallengeSolutionArgs;
    use crate::day_1::run_sonar_sliding_window;
    use crate::day_1::run_sonar_sweep_depth;

    use anyhow::{ Context, Result };

    const SONAR_SWEEP_DEPTH_SUBCOMMAND: &str = "1_1";
    const SONAR_SLIDING_WINDOW_SUBCOMMAND: &str = "1_2";

    fn parse_value(read_result: std::io::Result<String>) -> Result<isize> {
        let line = read_result.with_context(|| format!("Error while reading a line of file."))?;

        line.parse::<isize>().with_context(|| format!("Can't parse line '{}' to isize.", line))
    }

    pub struct SonarSweepDepth { }

    impl ChallengeSolutionArgs for SonarSweepDepth {
        fn get_subcommand(&self) -> &'static str {
            SONAR_SWEEP_DEPTH_SUBCOMMAND
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> Result<String> { 
            let input = read_input_from_matches(self, matches)?.map(parse_value);

            run_sonar_sweep_depth(input)
        }
    }

    pub struct SonarSlidingWindow { }

    impl ChallengeSolutionArgs for SonarSlidingWindow {
        fn get_subcommand(&self) -> &'static str {
            SONAR_SLIDING_WINDOW_SUBCOMMAND
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> Result<String> { 
            let input = read_input_from_matches(self, matches)?.map(parse_value);

            run_sonar_sliding_window(input)
        }
    }
}

pub fn run_sonar_sweep_depth<'a>(input: impl IntoIterator<Item = Result<isize>>) -> Result<String>{
    let mut previous_value_option: Option<isize> = None;
    let mut increased_counter: isize = 0;

    for read in input {
        let value = read?;

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

pub fn run_sonar_sliding_window(input: impl IntoIterator<Item = Result<isize>>) -> Result<String> {
    let mut previous_window_option: Option<isize> = None;

    let mut incomplete_windows: VecDeque<isize> = VecDeque::new();
    let mut increased_counter: isize = 0;

    for read in input {
        let value = read?;

        incomplete_windows.iter_mut().for_each(|e| *e += value);
        incomplete_windows.push_back(value);

        if incomplete_windows.len() >= 3 {
            // Two other windows were added since the window at the front was pushed
            // Meaning the window at the front is the sum of the 3 value, which makes it complete.
            let window = incomplete_windows.pop_front().unwrap();

            if let Some(previous_window) = previous_window_option {
                if window > previous_window {
                    increased_counter += 1;
                }
            }

            previous_window_option = Some(window);
        }
    }

    Ok(format!("Answer: {}", increased_counter))

}