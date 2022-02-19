pub mod cli {
    use crate::day_5::run_hydrotermal_diagonals;
    use crate::command_line::ChallengeSolutionArgs;
    use crate::day_5::run_hydrotermal_venture;
    use crate::command_line::read_input_from_matches;

    const HYDROTERMAL_VENTURE: &str = "5_1";
    const HYDRO_DIAGONAL: &str = "5_2";
    pub struct HydrotermalVenture { }

    impl ChallengeSolutionArgs for HydrotermalVenture {
        fn get_subcommand(&self) -> &'static str {
            HYDROTERMAL_VENTURE
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> anyhow::Result<String> { 
            let input = read_input_from_matches(self, matches)?;
            
            let answer = run_hydrotermal_venture(input);

            Ok(format!("Answer is: {}", answer?))
        }
    }

    pub struct HydroDiagonal { }

    impl ChallengeSolutionArgs for HydroDiagonal {
        fn get_subcommand(&self) -> &'static str {
            HYDRO_DIAGONAL
        }

        fn run(&mut self, matches: &clap::ArgMatches) -> anyhow::Result<String> { 
            let input = read_input_from_matches(self, matches)?;
            
            let answer = run_hydrotermal_diagonals(input);

            Ok(format!("Answer is: {}", answer?))
        }
    }
}


const FLOOR_WIDTH: usize = 1000;
const FLOOR_HEIGHT: usize = 1000;
type OceanFloor = Box<[u8; FLOOR_WIDTH * FLOOR_HEIGHT]>;
type PointCoord = u16;
type Point = [PointCoord; 2];

fn new_floor() -> OceanFloor {
    Box::new([0; FLOOR_WIDTH * FLOOR_HEIGHT])
}

fn point_to_pos(x: usize, y: usize) -> usize {
    y * FLOOR_WIDTH + x
}

// Returns number of overlaps
fn horizontal_line(floor: &mut OceanFloor, x: PointCoord, mut y1: PointCoord, mut y2: PointCoord) -> usize {
    if y1 > y2 {
        std::mem::swap(&mut y1, &mut y2);
    }
    
    (y1..y2+1).fold(0, |acc, y| {
        let val = &mut floor[point_to_pos(x as usize, y as usize)];
        match val {
            0 => { *val += 1; acc },
            1 => { *val += 1; acc + 1 },
            2 => { acc }
            _ => panic!("Line was marked twice")
        }
    })
}

// Returns number of overlaps
fn vertical_line(floor: &mut OceanFloor, y: PointCoord, mut x1: PointCoord, mut x2: PointCoord) -> usize {
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
    }

    (x1..x2+1).fold(0, |acc, x| {
        let val = &mut floor[point_to_pos(x as usize, y as usize)];
        match val {
            0 => { *val += 1; acc },
            1 => { *val += 1; acc + 1 },
            2 => { acc }
            _ => panic!("Line was marked twice")
        }
    })
}

fn diagonal(floor: &mut OceanFloor, p1: Point, p2: Point) -> usize {
    let mut start = p1[0];
    let mut end = p2[0];
    if start > end { std::mem::swap(&mut start, &mut end); }

    let inc_x: isize = if p1[0] > p2[0] { -1 } else { 1 };
    let inc_y: isize = if p1[1] > p2[1] { -1 } else { 1 };

    let mut acc = 0;
    let mut x = p1[0] as isize;
    let mut y = p1[1] as isize;

    let x2 = p2[0] as isize;
    while x != x2 {
        let val = &mut floor[point_to_pos(x as usize, y as usize)];

        acc += match val {
            0 => { *val += 1; 0 },
            1 => { *val += 1; 1 },
            2 => { 0 }
            _ => panic!("Line was marked twice")
        };
        
        x += inc_x;
        y += inc_y;
    }

    acc
}

fn read_line_to_pair_points(line: &str) -> [Point; 2] {
    line
        .split(" -> ").collect::<arrayvec::ArrayVec<_, 2>>().iter()
        .map(|point| {
            point.split(',')
                .map(|val| val.parse::<PointCoord>().unwrap())
                .collect::<arrayvec::ArrayVec<_, 2>>().into_inner()
        })
        .collect::<Result<arrayvec::ArrayVec<_, 2>, _>>().unwrap().into_inner().unwrap()
}

pub fn run_hydrotermal_venture<S: AsRef<str>>(input: impl IntoIterator<Item = std::io::Result<S>>) -> anyhow::Result<usize> {
    let mut floor = new_floor();

    let overlaps: std::io::Result<usize> = input.into_iter().map(|result| {
        let line = result?;

        if line.as_ref().is_empty() { return Ok(0) };

        let [p1, p2] = read_line_to_pair_points(line.as_ref());
        
        let overlaps = {
            if p1[0] == p2[0] { horizontal_line(&mut floor, p1[0], p1[1], p2[1]) }
            else if p1[1] == p2[1] { vertical_line(&mut floor, p1[1], p1[0], p2[0]) }
            else { 0 }
        };

        Ok(overlaps)
    }).try_fold(0, |acc, x: std::io::Result<usize>| Ok(acc+x?));

    Ok(overlaps?)
}

pub fn run_hydrotermal_diagonals<S: AsRef<str>>(input: impl IntoIterator<Item = std::io::Result<S>>) -> anyhow::Result<usize> {
    let mut floor = new_floor();

    let overlaps: std::io::Result<usize> = input.into_iter().map(|result| {
        let line = result?;

        if line.as_ref().is_empty() { return Ok(0) };

        let [p1, p2]: [Point; 2] = read_line_to_pair_points(line.as_ref());
        
        let overlaps = {
            if p1[0] == p2[0] { horizontal_line(&mut floor, p1[0], p1[1], p2[1]) }
            else if p1[1] == p2[1] { vertical_line(&mut floor, p1[1], p1[0], p2[0]) }
            else { diagonal(&mut floor, p1, p2) }
        };

        Ok(overlaps)
    }).try_fold(0, |acc, x: std::io::Result<usize>| Ok(acc+x?));

    Ok(overlaps?)
}



#[cfg(test)]
mod test {
    use crate::day_5::run_hydrotermal_diagonals;
    use crate::day_5::run_hydrotermal_venture;

    const EXAMPLE: &str = "0,9 -> 5,9\n\
        8,0 -> 0,8\n\
        9,4 -> 3,4\n\
        2,2 -> 2,1\n\
        7,0 -> 7,4\n\
        6,4 -> 2,0\n\
        0,9 -> 2,9\n\
        3,4 -> 1,4\n\
        0,0 -> 8,8\n\
        5,5 -> 8,2\n";

    #[test]
    fn it_passes_hydrotermal_venture_example() {
        let answer = run_hydrotermal_venture(EXAMPLE.split('\n').map(|l| Ok(l)));

        assert_eq!(5, answer.unwrap());
    }

    #[test]
    fn it_passes_hydrotermal_diagonals() {
        let answer = run_hydrotermal_diagonals(EXAMPLE.split('\n').map(|l| Ok(l)));

        assert_eq!(12, answer.unwrap());
    }
}