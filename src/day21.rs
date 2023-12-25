use crate::read_lines;

pub fn day21() {
    let path = "data/day21.txt";
    let count = part1(path, 64);
    println!("Day 21 Part 1: {}", count);
    let count = part2(path);
    println!("Day 21 Part 2: {}", count);
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

    return stack.len();
}

fn part2(path: &str) -> usize {
    let lines = read_lines(path);

    let (grid, (start_i, start_j)) = parse_input(&lines);

    let result = tokio::task::block_in_place(|| {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        runtime.block_on(async {
            let one_handle = simulate_part_2(&grid, start_i, start_j, 65);
            let two_handle = simulate_part_2(&grid, start_i, start_j, 196);
            let three_handle = simulate_part_2(&grid, start_i, start_j, 327);

            let one = one_handle.await as i64;
            let two = two_handle.await as i64;
            let three = three_handle.await as i64;

            println!("One: {}", one);
            println!("Two: {}", two);
            println!("Three: {}", three);

            let a = (three - (2 * two) + one) / 2;
            let b = two - one - a;
            let c = one;

            let n = (26_501_365 - 65) / 131;

            println!("a: {}, b: {}, c: {}, n: {}", a, b, c, n);

            (a * (n * n)) + (b * n) + c
        })
    });

    return result as usize;
}

async fn simulate_part_2(
    grid: &Vec<Vec<Garden>>,
    start_i: usize,
    start_j: usize,
    steps: usize,
) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let iheight = height as i64;
    let iwidth = width as i64;

    let mut stack = vec![(start_i as i64, start_j as i64)];

    for _ in 0..steps {
        let mut tmp_stack = vec![];

        while let Some((i, j)) = stack.pop() {
            // NOrth

            let new_i = i - 1;

            let (grid_x, grid_y) = mod_translate(new_i as i64, j as i64, iwidth, iheight);

            if grid[grid_x][grid_y] == Garden::Plot {
                if !tmp_stack.contains(&(new_i, j)) {
                    tmp_stack.push((new_i, j));
                }
            }

            // East

            let new_j = j + 1;

            let (grid_x, grid_y) = mod_translate(i as i64, new_j as i64, iwidth, iheight);

            if grid[grid_x][grid_y] == Garden::Plot {
                if !tmp_stack.contains(&(i, new_j)) {
                    tmp_stack.push((i, new_j));
                }
            }

            // South

            let new_i = i + 1;

            let (grid_x, grid_y) = mod_translate(new_i as i64, j as i64, iwidth, iheight);

            if grid[grid_x][grid_y] == Garden::Plot {
                if !tmp_stack.contains(&(new_i, j)) {
                    tmp_stack.push((new_i, j));
                }
            }

            // West

            let new_j = j - 1;

            let (grid_x, grid_y) = mod_translate(i as i64, new_j as i64, iwidth, iheight);

            if grid[grid_x][grid_y] == Garden::Plot {
                if !tmp_stack.contains(&(i, new_j)) {
                    tmp_stack.push((i, new_j));
                }
            }
        }

        stack = tmp_stack
    }

    return stack.len();
}

fn mod_translate(x: i64, y: i64, width: i64, height: i64) -> (usize, usize) {
    fn custom_mod(num: i64, len: i64) -> i64 {
        return (num % len + len) % len;
    }

    return (
        custom_mod(x, width) as usize,
        custom_mod(y, height) as usize,
    );
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

#[test]
fn test_mod() {
    let height = 3;
    let width = 3;

    let ans = mod_translate(-1, 0, height, width);

    assert_eq!(ans, (2, 0));

    let ans = mod_translate(-4, 0, height, width);

    assert_eq!(ans, (2, 0));
}
