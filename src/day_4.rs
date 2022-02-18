pub mod cli {
    use crate::day_4::run_losing_board;
    use crate::day_4::run_giant_squid;
    use crate::day_4::read_puzzle_input;
    use crate::command_line::read_input_from_matches;
    use crate::command_line::ChallengeSolutionArgs;
    use anyhow::Result;

    const GIANT_SQUID_COMMAND: &str = "4_1";
    const LOSING_BOARD_COMMAND: &str = "4_2";

    pub struct GiantSquid { }

    impl ChallengeSolutionArgs for GiantSquid {
        fn get_subcommand(&self) -> &'static str {
            GIANT_SQUID_COMMAND
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> Result<String> { 
            let input = read_input_from_matches(self, matches)?;
            let puzzle = read_puzzle_input(input)?;
            
            let answer = run_giant_squid(puzzle);

            Ok(format!("Answer is: {}", answer))
        }
    }

    pub struct LosingBoard { }

    impl ChallengeSolutionArgs for LosingBoard {
        fn get_subcommand(&self) -> &'static str {
            LOSING_BOARD_COMMAND
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> Result<String> { 
            let input = read_input_from_matches(self, matches)?;
            let puzzle = read_puzzle_input(input)?;
            
            let answer = run_losing_board(puzzle);

            Ok(format!("Answer is: {}", answer))
        }
    }
}

const CARD_WIDTH: usize = 5;
const CARD_HEIGHT: usize = 5;

pub struct BingoCard {
    uncrossed_sum: u32,
    // Association of a value on a bingo card to a tuple of its column and row positions
    value_map: std::collections::HashMap<u8, (usize, usize)>,
    uncrossed_columns: [u8; 5],
    uncrossed_rows: [u8; 5],
}

impl BingoCard {
    fn new(values: &[u8; 25]) -> BingoCard {
        let mut card = BingoCard {
            uncrossed_sum: 0,
            value_map: std::collections::HashMap::new(),
            uncrossed_columns: [5; 5],
            uncrossed_rows: [5; 5]
        };

        for (i, val) in values.iter().enumerate() {
            card.uncrossed_sum += *val as u32;
            let (column, row) = (i % CARD_HEIGHT, i / CARD_WIDTH);
            card.value_map.insert(*val, (column, row));
        }

        card
    }

    // Returns score if a bingo is found
    fn cross_value(&mut self, value: u8) -> Option<u32> {
        let (column, row) = {
            match self.value_map.get(&value) {
                Some(value_pos) => *value_pos,
                None => return None,
            }
        };

        self.uncrossed_sum -= value as u32;
        
        self.uncrossed_columns[column] -= 1;
        self.uncrossed_rows[row] -= 1;

        if self.uncrossed_columns[column] == 0 || self.uncrossed_rows[row] == 0 {
            return Some(self.calculate_bingo(value as u32));
        }

        return None;
    }

    fn calculate_bingo(&self, value: u32) -> u32 {
        self.uncrossed_sum * value
    }
}

pub struct PuzzleInput {
    values: Vec<u8>,
    cards: Vec<BingoCard>,
}

struct BingoCardAcc {
    vec: Vec<BingoCard>,
    tmp_values: [u8; 25],
    i: usize
}

impl BingoCardAcc {
    fn new() -> Self {
        return Self {
            vec: Vec::new(),
            tmp_values: [0; 25],
            i: 0
        }
    }

    fn flush_values(&mut self) {
        if self.i != 0 {
            self.vec.push(BingoCard::new(&self.tmp_values));
            self.i = 0;
        }
    } 

    fn add_row(&mut self, row: Vec<u8>) {
        self.tmp_values[self.i*CARD_WIDTH..self.i*CARD_WIDTH+CARD_WIDTH].copy_from_slice(&row);
        self.i += 1;
    }
}

pub fn read_puzzle_input<S: AsRef<str>>(input: impl IntoIterator<Item = std::io::Result<S>>) -> anyhow::Result<PuzzleInput>  {
    use anyhow::Context;

    let mut input_iter = input.into_iter();

    let first_line = input_iter.next().ok_or(anyhow::anyhow!("No input was given"))?.with_context(|| "Failed to read a first line")?;
    let values: Vec<u8> = first_line.as_ref().split(',')
        .map(|substring| substring.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()?; // One day this will by try_collect(). Hopefully

    input_iter.next(); // Remove newline
    let mut card_acc: BingoCardAcc = input_iter.fold(Ok(BingoCardAcc::new()), |acc_res: anyhow::Result<BingoCardAcc>, result| {
        let mut acc = acc_res?;
        let asref = result?;
        let line: &str = asref.as_ref();

        if line.is_empty() {
            acc.flush_values();
        } else {
            let row = line.split(' ').filter(|word| !word.is_empty())
                .map(|substring| substring.parse::<u8>())
                .collect::<Result<Vec<_>, _>>()?;

            acc.add_row(row);
        }

        Ok(acc)
    })?;

    card_acc.flush_values();

    Ok(PuzzleInput { values: values, cards: card_acc.vec })
}

fn run_giant_squid(input: PuzzleInput) -> u32 {
    let values = input.values;
    let mut cards = input.cards;

    let answer = values.into_iter().find_map(|value| {
        cards.iter_mut().find_map(|card| card.cross_value(value))
    }).unwrap();

    return answer;
}

pub fn run_losing_board(input: PuzzleInput) -> u32 {
    use retain_mut::RetainMut;

    let values = input.values;
    let mut cards = input.cards;
    let mut scores: Vec<u32> = Vec::new();

    let answer = values.into_iter().find_map(|value| {
        scores.clear();

        cards.retain_mut(|card| {
            match card.cross_value(value) {
                None => true,
                Some(bingo) => {
                    scores.push(bingo);
                    false
                }
            }
        });

        match cards.is_empty() {
            false => None,
            true => {
                Some(*scores.last().unwrap())
            }
        }
    }).unwrap();

    answer
}

#[cfg(test)]
mod test {
    use crate::day_4::run_losing_board;
    use crate::day_4::run_giant_squid;
    use crate::day_4::read_puzzle_input;

const EXAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
\n\
        22 13 17 11  0\n\
        8  2 23  4 24\n\
        21  9 14 16  7\n\
        6 10  3 18  5\n\
        1 12 20 15 19\n\
\n\
        3 15  0  2 22\n\
        9 18 13 17  5\n\
        19  8  7 25 23\n\
        20 11 10 24  4\n\
        14 21 16 12  6\n\
\n\
        14 21 17 24  4\n\
        10 16 15  9 19\n\
        18  8 23 26 20\n\
        22 11 13  6  5\n\
        2  0 12  3  7";

    #[test]
    fn it_passes_giant_squid_example() {
        let puzzle_input = read_puzzle_input(EXAMPLE.split('\n').map(|s| Ok(s))).unwrap();
        let answer = run_giant_squid(puzzle_input);
        assert_eq!(answer, 4512);
    }

    #[test]
    fn it_passes_losing_board_example() {
        let puzzle_input = read_puzzle_input(EXAMPLE.split('\n').map(|s| Ok(s))).unwrap();
        let answer = run_losing_board(puzzle_input);
        assert_eq!(answer, 1924);
    }
}
