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
        
            run_binary_diagnostic(input).map(|r| r.to_string())
        }
    }
}

pub fn run_binary_diagnostic<S: AsRef<str>>(input: impl IntoIterator<Item = std::io::Result<S>>) -> anyhow::Result<isize> {
    #[derive(Clone, Copy)]
    struct BitCount {
        zero: usize,
        one: usize,
    }

    // Create an array of counts by peeking the first line, and finding out the size of the input.
    let mut input = input.into_iter().peekable();

    let line_length: usize = {
        let result = input.peek();
        let peeked_line = match result {
            Some(result) => result.as_ref().map_err(|e| anyhow::anyhow!("Failed to read the first line. {}", e))?,
            None => return Err(anyhow::anyhow!("Input is empty")),
        };

        peeked_line.as_ref().len()
    };

    let mut counts = vec![BitCount { zero: 0, one: 0}; line_length];

    for result in input {
        let line = result?;

        for (i, bit) in line.as_ref().chars().take(line_length).enumerate() {
            match bit {
                '0' => counts[i].zero += 1,
                '1' => counts[i].one += 1,
                _ => return Err(anyhow::anyhow!("Invalid character '{}' at line '{}'", bit, line.as_ref()))
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for (i, bitcount) in counts.iter().enumerate() {
        match bitcount.zero > bitcount.one {
            true => epsilon += 1 << line_length - i - 1,
            false => gamma += 1 << line_length - i - 1,
        }
    }

    Ok(gamma * epsilon)
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
        let result = run_binary_diagnostic(DIAGNOSTIC_REPORT_EXAMPLE.split('\n').map(|i| Ok(i)));

        assert_eq!(result.map_err(|e| format!("Failed to calculate power consumption. {}", e)).unwrap(), 198);
    }
}