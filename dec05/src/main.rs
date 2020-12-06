use std::str::FromStr;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let boarding_passes: Vec<BoardingPass> = io::stdin()
        .lock()
        .lines()
        .map(|line| BoardingPass::from_str(&line.unwrap()).unwrap())
        .collect();

    let mut rows: HashMap<usize, Vec<BoardingPass>> = HashMap::new();
    for bp in boarding_passes {
        if let Some(row) = rows.get_mut(&bp.row) {
            row.push(bp);
        } else {
            rows.insert(bp.row.to_owned(), vec![bp]);
        }
    }

    for (row, boarding_passes) in rows {
        eprintln!("row: {} has {} passengers!", row, boarding_passes.len());
    }
}

#[derive(Debug, PartialEq)]
struct BoardingPass {
    row: usize,
    column: usize,
}

impl BoardingPass {
    fn seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

fn upper_or_lower_processing(min: &mut usize, max: &mut usize, input: char) {
    match input {
        // Take the upper half
        'B' | 'R' => {
            *min = ((*max + 1) - *min) / 2 + *min;
        },

        // Take the lower half
        'F' | 'L' => {
            *max = (*max - *min) / 2 + *min;
        },

        _ => panic!("Unregonized instruction"),
    }
}

fn find_seat(partition: &str, boundry: &mut (usize, usize)) -> usize {
    for letter in partition.chars() {
        upper_or_lower_processing(
            &mut boundry.0,
            &mut boundry.1,
            letter,
        );
    }

    if boundry.0 != boundry.1 {
        panic!("Unable to find position");
    }

    boundry.0
}

impl FromStr for BoardingPass {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rows = &input[0..=6];
        let columns = &input[7..=9];

        let row = find_seat(rows, &mut (0, 127));
        let column = find_seat(columns, &mut (0, 7));

        Ok(BoardingPass {
            row,
            column,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_upper_lower_half_instruction() {
        let mut min = 0;
        let mut max = 127;

        upper_or_lower_processing(&mut min, &mut max, 'F');
        assert_eq!((min, max), (0, 63));

        upper_or_lower_processing(&mut min, &mut max, 'B');
        assert_eq!((min, max), (32, 63));

        upper_or_lower_processing(&mut min, &mut max, 'F');
        assert_eq!((min, max), (32, 47));

        upper_or_lower_processing(&mut min, &mut max, 'B');
        assert_eq!((min, max), (40, 47));

        upper_or_lower_processing(&mut min, &mut max, 'B');
        assert_eq!((min, max), (44, 47));

        upper_or_lower_processing(&mut min, &mut max, 'F');
        assert_eq!((min, max), (44, 45));

        upper_or_lower_processing(&mut min, &mut max, 'F');
        assert_eq!((min, max), (44, 44));

        let mut min = 0;
        let mut max = 7;
        upper_or_lower_processing(&mut min, &mut max, 'R');
        assert_eq!((min, max), (4, 7));
    }

    #[test]
    fn test_parse_binary_space_partition() {
        assert_eq!(BoardingPass::from_str("FBFBBFFRLR"), Ok(BoardingPass { row: 44, column: 5 }));
        assert_eq!(BoardingPass::from_str("BFFFBBFRRR"), Ok(BoardingPass { row: 70, column: 7 }));
        assert_eq!(BoardingPass::from_str("FFFBBBFRRR"), Ok(BoardingPass { row: 14, column: 7 }));
        assert_eq!(BoardingPass::from_str("BBFFBBFRLL"), Ok(BoardingPass { row: 102, column: 4 }));
    }
}
