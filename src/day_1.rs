use std::collections::VecDeque;

pub mod cli {
    use crate::command_line::get_input_path;
    use crate::command_line::expect_submatches;
    use crate::command_line::add_input;
    use crate::command_line::ChallengeSolutionArgs;
    use crate::day_1::run_sonar_sliding_window;
    use crate::day_1::run_sonar_sweep_depth;
    use crate::core::file::read_lines;

    use crate::core::Result;

    const SONAR_SWEEP_DEPTH_SUBCOMMAND: &str = "1_1";
    const SONAR_SLIDING_WINDOW_SUBCOMMAND: &str = "1_2";

    fn parse_value(read_result: std::io::Result<String>) -> Result<isize> {
        let line = read_result.map_err(|err| format!("Error while reading line of file: {}", err))?;

        line.parse::<isize>().map_err(|err| format!("Can't parse line '{}' to isize. {}", line, err))
    }

    pub struct SonarSweepDepth { }

    impl ChallengeSolutionArgs for SonarSweepDepth {
        fn add_subcommand<'a>(&self, app: clap::App<'a>) -> (&'static str, clap::App<'a>) {
            (SONAR_SWEEP_DEPTH_SUBCOMMAND, add_input(app, SONAR_SWEEP_DEPTH_SUBCOMMAND))
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> Result<String> { 
            let submatches = expect_submatches(matches, SONAR_SWEEP_DEPTH_SUBCOMMAND);

            let input_path = get_input_path(submatches)?;

            let input = read_lines(input_path)?.map(parse_value);

            run_sonar_sweep_depth(input)
        }
    }

    pub struct SonarSlidingWindow { }

    impl ChallengeSolutionArgs for SonarSlidingWindow {
        fn add_subcommand<'a>(&self, app: clap::App<'a>) -> (&'static str, clap::App<'a>) {
            (SONAR_SLIDING_WINDOW_SUBCOMMAND, add_input(app, SONAR_SLIDING_WINDOW_SUBCOMMAND))
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> Result<String> { 
            let submatches = expect_submatches(matches, SONAR_SLIDING_WINDOW_SUBCOMMAND);

            let input_path = get_input_path(submatches)?;

            let input = read_lines(input_path)?.map(parse_value);

            run_sonar_sliding_window(input)
        }
    }
}

pub fn run_sonar_sweep_depth<'a>(input: impl IntoIterator<Item = Result<isize, String>>) -> Result<String, String>{
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

pub fn run_sonar_sliding_window(input: impl IntoIterator<Item = Result<isize, String>>) -> Result<String, String> {
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