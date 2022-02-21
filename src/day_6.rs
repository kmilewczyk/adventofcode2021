pub mod cli {
    use crate::day_6::run_unlimited_lanternfish;
use crate::day_6::run_lanternfish;
    use crate::command_line::ChallengeSolutionArgs;
    use crate::command_line::read_input_from_matches;

    const LANTERNFISH: &str = "6_1";
    const UNLIMITED_LANTERNFISH: &str = "6_2";
    pub struct LanternFish { }

    impl ChallengeSolutionArgs for LanternFish {
        fn get_subcommand(&self) -> &'static str {
            LANTERNFISH
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> anyhow::Result<String> { 
            let input: Vec<usize> = read_input_from_matches(self, matches)?
                .into_iter().next().ok_or(anyhow::anyhow!("Input is empty"))??
                .split(',').map(|word| word.trim_end().parse::<usize>().unwrap()).collect();

            
            let answer = run_lanternfish(input);

            Ok(format!("Answer is: {}", answer))
        }
    }

    pub struct UnlimitedLanternfish { }

    impl ChallengeSolutionArgs for UnlimitedLanternfish {
        fn get_subcommand(&self) -> &'static str {
            UNLIMITED_LANTERNFISH
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> anyhow::Result<String> { 
            let input: Vec<usize> = read_input_from_matches(self, matches)?
                .into_iter().next().ok_or(anyhow::anyhow!("Input is empty"))??
                .split(',').map(|word| word.trim_end().parse::<usize>().unwrap()).collect();

            
            let answer = run_unlimited_lanternfish(input);

            Ok(format!("Answer is: {}", answer))
        }
    }
}
type FishInt = u64;
const LANTERNFISH_DAY_LIFECYCLE: usize = 9;
const LANTERNFISH_RESET_TIMER: usize = 6;

pub fn run_lanternfish(input: Vec<usize>) -> FishInt {
    let mut day_schedule = input.into_iter().fold([0 as FishInt; LANTERNFISH_DAY_LIFECYCLE], |mut acc, fish| {
        acc[fish] += 1;
        acc
    });

    for _ in 0..80 {
        let birthing_lanternfishes = day_schedule[0];
        day_schedule.copy_within(1..LANTERNFISH_DAY_LIFECYCLE, 0);
        day_schedule[LANTERNFISH_DAY_LIFECYCLE-1] = birthing_lanternfishes;
        day_schedule[LANTERNFISH_RESET_TIMER] += birthing_lanternfishes;
    }


    day_schedule.iter().sum::<FishInt>() 
}

pub fn run_unlimited_lanternfish(input: Vec<usize>) -> FishInt {
    let mut day_schedule = input.into_iter().fold([0 as FishInt; LANTERNFISH_DAY_LIFECYCLE], |mut acc, fish| {
        acc[fish] += 1;
        acc
    });

    for _ in 0..256 {
        let birthing_lanternfishes = day_schedule[0];
        day_schedule.copy_within(1..LANTERNFISH_DAY_LIFECYCLE, 0);
        day_schedule[LANTERNFISH_DAY_LIFECYCLE-1] = birthing_lanternfishes;
        day_schedule[LANTERNFISH_RESET_TIMER] += birthing_lanternfishes;
    }


    day_schedule.iter().sum::<FishInt>()
}

#[cfg(test)]
mod test {
    use crate::day_6::run_unlimited_lanternfish;
use crate::day_6::run_lanternfish;

const EXAMPLE: &str = "3,4,3,1,2\n";

    #[test]
    fn it_passes_lanternfish_example() {
        let answer = run_lanternfish(EXAMPLE.split(',').map(|word| word.trim().parse::<usize>().unwrap()).collect());
        assert_eq!(answer, 5934)
    }

    #[test]
    fn it_passes_unlimited_lanternfish_example() {
        let answer = run_unlimited_lanternfish(EXAMPLE.split(',').map(|word| word.trim().parse::<usize>().unwrap()).collect());
        assert_eq!(answer, 26984457539);
    }
}