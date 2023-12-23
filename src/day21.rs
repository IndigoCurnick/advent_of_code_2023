use crate::read_lines;

pub fn day21() {
    let path = "data/day21.txt";
    let count = part1(path, 64);
    println!("Day 21 Part 1: {}", count);
}

fn part1(path: &str, steps: usize) -> usize {
    let lines = read_lines(path);

    let (grid, (start_i, start_j)) = parse_input(&lines);

    let height = grid.len();
    let width = grid[0].len();

    let mut stack = vec![(start_i, start_j)];

    for _ in 0..steps {
        let mut tmp_stack = vec![];

        while let Some((i, j)) = stack.pop() {
            // NOrth
            if i > 0 {
                let new_i = i - 1;

                if grid[new_i][j] == Garden::Plot {
                    if !tmp_stack.contains(&(new_i, j)) {
                        tmp_stack.push((new_i, j));
                    }
                }
            }

            // East

            if j < width - 1 {
                let new_j = j + 1;

                if grid[i][new_j] == Garden::Plot {
                    if !tmp_stack.contains(&(i, new_j)) {
                        tmp_stack.push((i, new_j));
                    }
                }
            }

            // South
            if i < height - 1 {
                let new_i = i + 1;

                if grid[new_i][j] == Garden::Plot {
                    if !tmp_stack.contains(&(new_i, j)) {
                        tmp_stack.push((new_i, j));
                    }
                }
            }

            // West

            if j > 0 {
                let new_j = j - 1;

                if grid[i][new_j] == Garden::Plot {
                    if !tmp_stack.contains(&(i, new_j)) {
                        tmp_stack.push((i, new_j));
                    }
                }
            }
        }

        stack = tmp_stack
    }

    display(&grid, &stack);
    println!("Current Stack: {:?}", stack);

    return stack.len();
}

fn display(grid: &Vec<Vec<Garden>>, stack: &Vec<(usize, usize)>) {
    for (i, line) in grid.iter().enumerate() {
        for (j, garden) in line.iter().enumerate() {
            if stack.contains(&(i, j)) {
                print!("@");
            } else if *garden == Garden::Plot {
                print!(".");
            } else if *garden == Garden::Rocks {
                print!("#");
            }
        }
        print!("\n");
    }
}

fn parse_input(lines: &Vec<String>) -> (Vec<Vec<Garden>>, (usize, usize)) {
    let mut bigi = 0;
    let mut bigj = 0;

    let mut out = vec![];

    for (i, line) in lines.iter().enumerate() {
        let mut tmp = vec![];

        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                bigi = i;
                bigj = j;
            }

            tmp.push(Garden::from(c));
        }

        out.push(tmp);
    }

    return (out, (bigi, bigj));
}

#[derive(PartialEq)]
enum Garden {
    Plot,
    Rocks,
}

impl From<char> for Garden {
    fn from(value: char) -> Self {
        if value == 'S' || value == '.' {
            return Self::Plot;
        } else if value == '#' {
            return Self::Rocks;
        } else {
            panic!("Unknown input `{}`", value);
        }
    }
}

#[test]
fn test_part1() {
    let path = "data/day21_demo.txt";
    let count = part1(path, 6);
    assert_eq!(count, 16);
}
