pub mod cli {
    use crate::day_3::run_life_support_rating;
use crate::day_3::read_input_to_vec;
use crate::command_line::read_input_from_matches;
    use crate::day_3::run_binary_diagnostic;
    use crate::command_line::ChallengeSolutionArgs;
    use anyhow::Result;

    const BINARY_DIAGNOSTIC_SUBCOMMAND: &str = "3_1";
    const LIFE_SUPPORT_RATING_SUBCOMMAND: &str = "3_2";

    pub struct BinaryDiagnostic { }

    impl ChallengeSolutionArgs for BinaryDiagnostic {
        fn get_subcommand(&self) -> &'static str {
            BINARY_DIAGNOSTIC_SUBCOMMAND
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> Result<String> { 
            let input = read_input_from_matches(self, matches)?;
            let puzzle = read_input_to_vec(input)?;
            
            let answer = run_binary_diagnostic(puzzle);

            Ok(format!("Answer is: {}", answer))
        }
    }

    pub struct LifeSupportRating { }

    impl ChallengeSolutionArgs for LifeSupportRating {
        fn get_subcommand(&self) -> &'static str {
            LIFE_SUPPORT_RATING_SUBCOMMAND
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> Result<String> { 
            let input = read_input_from_matches(self, matches)?;
            let puzzle = read_input_to_vec(input)?;
            
            let answer = run_life_support_rating(puzzle)?;

            Ok(format!("Answer is: {}", answer))
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

fn count_ones<'a>(input: impl IntoIterator<Item = &'a PuzzleBinary>, pos: usize) -> usize{
    input.into_iter().filter(|&number| number & (1 << pos) != 0).count()
}

pub fn run_binary_diagnostic(puzzle: PuzzleInput) -> usize {
    let gamma = (0..puzzle.binary_size)
        .map(|i| {
            // Add up all the ones at the position i
            let ones_count = count_ones(&puzzle.input, i);

            // If the bit is the majority then gamma has a bit set on i position
            // Also note to self: multiplication is faster than division
            if ones_count * 2 > puzzle.input.len()
               { 1 << i } else { 0 }
        })
        .fold(0, |acc, bit| { acc | bit });

    // Mask to ignore bits from negation
    let mask = (1 << puzzle.binary_size) - 1;

    gamma * (mask & !gamma)
}

pub fn run_life_support_rating(puzzle: PuzzleInput) -> anyhow::Result<PuzzleBinary> {
    let mut oxygen: PuzzleBinary = 0;
    let mut co2: PuzzleBinary = 0;

    let mut current = puzzle.input.clone();
    for i in (0..puzzle.binary_size).rev() {
        let ones_count = count_ones(&current, i);
        // Guartneed by Rust to be 1
        let oxygen_criteria = (ones_count * 2 >= current.len()) as PuzzleBinary;
        
        current.retain(|&num| (num >> i) & 1 == oxygen_criteria);

        if current.len() < 2 {
            oxygen = *current.get(0).ok_or(anyhow::anyhow!("There was no valid number filtering by oxygen criteria"))?;
            break;
        }
    }

    let mut current = puzzle.input;
    for i in (0..puzzle.binary_size).rev() {
        let ones_count = count_ones(&current, i);
        let co2_criteria = (ones_count * 2 < current.len()) as PuzzleBinary;
        
        current.retain(|&num| (num >> i) & 1 == co2_criteria);

        if current.len() < 2 {
            co2 = *current.get(0).ok_or(anyhow::anyhow!("There was no valid number filtering by co2 criteria"))?;
            break;
        }
    }

    Ok(co2 * oxygen)
}

#[cfg(test)]
mod tests {
    use crate::day_3::run_life_support_rating;
    use crate::day_3::read_input_to_vec;
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
        let puzzle_input = read_input_to_vec(DIAGNOSTIC_REPORT_EXAMPLE.split('\n').map(|i| Ok(i))).unwrap();
        let result = run_binary_diagnostic(puzzle_input);

        assert_eq!(result, 198);
    }

    #[test]
    fn it_passes_life_support_rating_example() {
        let puzzle_input = read_input_to_vec(DIAGNOSTIC_REPORT_EXAMPLE.split('\n').map(|i| Ok(i))).unwrap();
        let result = run_life_support_rating(puzzle_input).unwrap();

        assert_eq!(result, 230);
    }
}