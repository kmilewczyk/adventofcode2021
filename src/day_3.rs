pub mod cli {
    use crate::command_line::read_input_from_matches;
    use crate::day_3::run_binary_diagnostic;
    use crate::command_line::ChallengeSolutionArgs;
    use anyhow::Result;

    const BINARY_DIAGNOSTIC_SUBCOMMAND: &str = "3_1";

    pub struct BinaryDiagnostic { }

    impl ChallengeSolutionArgs for BinaryDiagnostic {
        fn get_subcommand(&self) -> &'static str {
            BINARY_DIAGNOSTIC_SUBCOMMAND
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> Result<String> { 
            let input = read_input_from_matches(self, matches)?;
        
            todo!()
        }
    }
}

pub type PuzzleBinary = u32;

pub struct PuzzleInput {
    pub input: Vec<PuzzleBinary>,
    pub binary_size: usize,
}

pub fn read_input_to_vec<S: AsRef<str>>(input: impl IntoIterator<Item = std::io::Result<S>>) -> anyhow::Result<PuzzleInput> {
    let mut input = input.into_iter().peekable();
    let line_length = {
        let peek = input.peek();
        match peek {
            Some(result) => result.as_ref().map_err(|e| anyhow::anyhow!("{}", e))?.as_ref().len(),
            None => return Err(anyhow::anyhow!("No lines to read")),
        }
    };

    let vec_result: std::io::Result<Vec<PuzzleBinary>> = input.into_iter()
        .map(|line| {
            let value = line?.as_ref().chars().enumerate().fold(0, |acc: PuzzleBinary, (i, c)| {
                if c == '1' {
                    acc | 1 << (line_length - i - 1)
                }
                else {
                    acc
                }
            });

            Ok(value)
        }).collect();
    
    
    Ok(PuzzleInput { input: vec_result?, binary_size: line_length })
}

pub fn run_binary_diagnostic(input: &[PuzzleBinary], number_size: usize) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day_3::run_binary_diagnostic;

    const DIAGNOSTIC_REPORT_EXAMPLE: &str = "\
        00100\n\
        11110\n\
        10110\n\
        10111\n\
        10101\n\
        01111\n\
        00111\n\
        11100\n\
        10000\n\
        11001\n\
        00010\n\
        01010";

    #[test]
    fn it_passes_binary_diagnostic_example() {
        // let result = run_binary_diagnostic(DIAGNOSTIC_REPORT_EXAMPLE.split('\n').map(|i| Ok(i)));

        // assert_eq!(result.map_err(|e| format!("Failed to calculate power consumption. {}", e)).unwrap(), 198);
    }
}