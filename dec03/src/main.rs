use std::io::{self, BufRead};
use std::convert::TryFrom;

fn main() {
    let stdin = io::stdin();

    let mut lines: usize = 0;
    let mut items: Vec<Slope> = vec![];

    for line in stdin.lock().lines() {
        lines += 1;
        for val in line.unwrap().chars() {
            items.push(Slope::try_from(val).unwrap());
        }
    }
    let pattern_width = items.len() / lines as usize;
    let map = Map {
        items,
        pattern_height: lines,
        pattern_width,
    };

    let slopes: &[(usize, usize)] = &[
        (1, 1),
        (1, 3),
        (1, 5),
        (1, 7),
        (2, 1),
    ];

    let mut multipled_trees = 1;

    for slope in slopes {
        multipled_trees *= traverse_map(&map, *slope).into_iter()
        .filter(|v| is_a_tree(v))
        .count();
    }

    eprintln!("Number of trees: {}", multipled_trees);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Slope {
    OpenSquare,
    Tree,
}

impl TryFrom<char> for Slope {
    type Error = &'static str;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            '.' => Ok(Slope::OpenSquare),
            '#' => Ok(Slope::Tree),
            _ => Err("Unknown slope"),
        }
    }
}

fn is_a_tree(slope: &Slope) -> bool {
    slope == &Slope::Tree
}

#[derive(Debug, PartialEq)]
struct Map {
    items: Vec<Slope>,
    pattern_width: usize,
    pattern_height: usize,
}

impl std::str::FromStr for Map {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines: usize = 0;
        let line = input.lines().next().unwrap().chars();
        let mut pattern_width: usize = 0;
        for _ in line {
            pattern_width = pattern_width + 1;
        }

        let items: Vec<Slope> = input
            .lines()
            .flat_map(|l| {
                lines = lines + 1;
                l.chars()
            })
            .map(|c| Slope::try_from(c).unwrap())
            .collect();

        Ok(Map {
            items,
            pattern_height: lines,
            pattern_width,
        })
    }
}

fn traverse_map(map: &Map, movements: (usize, usize)) -> Vec<Slope> {
    let mut reached_bottom = false;
    let mut result = vec![];
    let mut steps = vec![];

    for x in 1..=movements.1 {
        steps.push((0, 1));
    }
    for y in 1..=movements.0 {
        steps.push((map.pattern_width, 0));
    }

    //let movements = [
    //    (0, 1),
    //    (0, 1),
    //    (0, 1),
    //    (map.pattern_width, 0),
    //];
    let mut counter = 0;
    let mut x_pos = 0;
    let mut y_pos = 0;

    eprintln!("width: {}, height: {}", map.pattern_width, map.pattern_height);

    while reached_bottom == false {
        for movement in &steps {
            match movement {
                (0, sideway) => {
                    if x_pos >= map.pattern_width - 1 {
                        counter -= map.pattern_width - 1;
                        x_pos = 0;
                    } else {
                        counter += sideway;
                        x_pos += sideway;
                    }
                },
                (south, 0) => {
                    counter += south;
                    y_pos += 1;

                    if y_pos >= map.pattern_height - 1 {
                        reached_bottom = true;
                    }
                },
                (_, _) => panic!("Unmatched movement"),
            };
        }

        eprintln!("{}, xy: {}, {}, {}", counter, x_pos, y_pos, is_a_tree(&map.items[counter]));

        result.push(map.items[counter]);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "..##.......\r\n\
                              #...#...#..\r\n\
                              .#....#..#.\r\n\
                              ..#.#...#.#\r\n\
                              .#...##..#.\r\n\
                              ..#.##.....\r\n\
                              .#.#.#....#\r\n\
                              .#........#\r\n\
                              #.##...#...\r\n\
                              #...##....#\r\n\
                              .#..#...#.#";

    /// 1  ..##.......
    /// 2  #..O#...#..  14
    /// 3  .#....X..#.  28
    /// 4  ..#.#...#O#  42
    /// 5  .X...##..#.  45
    /// 6  ..#.X#.....  59
    /// 7  .#.#.#.O..#  72
    /// 8  .#........X  87
    /// 9  #.X#...#...  90
    /// 10 #...#X....#  104
    /// 11 .#..#...X.#  118
    ///
    /// 0  0   1   2   3   4   5   6   7   8   9   10
    /// 1  11  12  13  14  15  16  17  18  19  20  21
    /// 2  22  23  24  25  26  27  28  29  30  31  32
    /// 3  33  34  35  36  37  38  39  40  41  42  43
    /// 4  44  45  46  47  48  49  50  51  52  53  54
    /// 5  55  56  57  58  59  60  61  62  63  64  65
    /// 6  66  67  68  69  70  71  72  73  74  75  76
    /// 7  77  78  79  80  81  82  83  84  85  86  87
    /// 8  88  89  90  91  92  93  94  95  96  97  98
    /// 9  99  100 101 102 103 104 105 106 107 108 109
    /// 10 110 111 112 113 114 115 116 117 118 119 120


    #[test]
    fn test_parsing_input() {
        assert_eq!(Slope::try_from('#'), Ok(Slope::Tree));
        assert_eq!(Slope::try_from('.'), Ok(Slope::OpenSquare));
        assert_eq!(Slope::try_from('d'), Err("Unknown slope"));

        assert_eq!("..#\r\n##.".parse(), Ok(Map {
            items: vec![
                Slope::OpenSquare,
                Slope::OpenSquare,
                Slope::Tree,
                Slope::Tree,
                Slope::Tree,
                Slope::OpenSquare,
            ],
            pattern_width: 3,
            pattern_height: 2,
        }));
    }

    #[test]
    fn test_counting_trees() {
        let map = TEST_INPUT.parse().unwrap();
        assert_eq!(
            traverse_map(&map, (1, 3)),
            &[
              Slope::OpenSquare,
              Slope::Tree,
              Slope::OpenSquare,
              Slope::Tree,
              Slope::Tree,
              Slope::OpenSquare,
              Slope::Tree,
              Slope::Tree,
              Slope::Tree,
              Slope::Tree,
            ],
        );

        assert_eq!(
            traverse_map(&map, (1, 1)),
            &[
              Slope::OpenSquare,
              Slope::OpenSquare,
              Slope::OpenSquare,
              Slope::OpenSquare,
              Slope::Tree,
              Slope::OpenSquare,
              Slope::OpenSquare,
              Slope::OpenSquare,
              Slope::OpenSquare,
              Slope::Tree,
            ],
        );

        assert_eq!(
            traverse_map(&map, (1, 5)),
            &[
              Slope::OpenSquare,
              Slope::OpenSquare,
              Slope::Tree,
              Slope::Tree,
              Slope::OpenSquare,
              Slope::OpenSquare,
              Slope::OpenSquare,
              Slope::Tree,
              Slope::OpenSquare,
              Slope::OpenSquare,
            ],
        );

        assert_eq!(
            traverse_map(&map, (1, 7)),
            &[
              Slope::OpenSquare,
              Slope::OpenSquare,
              Slope::Tree,
              Slope::Tree,
              Slope::Tree,
              Slope::OpenSquare,
              Slope::OpenSquare,
              Slope::OpenSquare,
              Slope::OpenSquare,
              Slope::Tree,
            ],
        );
        assert_eq!(
            traverse_map(&map, (2, 1)),
            &[
              Slope::Tree,
              Slope::OpenSquare,
              Slope::Tree,
              Slope::OpenSquare,
              Slope::OpenSquare,
            ],
        );
    }
}
