use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

const DEBUG: bool = false;

#[derive(Debug, Clone)]
pub struct PuzzleInput {
    map: Vec<Vec<char>>,
    s_x: isize,
    s_y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn offset_to_dir(offset: (isize, isize)) -> Direction {
    use Direction::*;
    match offset {
        (0, -1) => Up,
        (0, 1) => Down,
        (1, 0) => Right,
        (-1, 0) => Left,
        _ => unreachable!(),
    }
}

/*

    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesnt show what shape the pipe has.

*/
fn does_pipe_fit(pipe_a: char, pipe_b: char, dir: (isize, isize)) -> bool {
    use Direction::*;
    let dir = offset_to_dir(dir); 

    match pipe_a {
        '|' => match dir {
            Up => match pipe_b {
                '|' | '7' | 'F' => true,
                _ => false,
            },
            Down => match pipe_b {
                '|' | 'J' | 'L' => true,
                _ => false,
            },
            _ => false,
        },
        '-' => match dir {
            Right => match pipe_b {
                '-' | '7' | 'J' => true,
                _ => false,
            },
            Left => match pipe_b {
                '-' | 'F' | 'L' => true,
                _ => false,
            },
            _ => false,
        },
        'L' => match dir {
            Up => match pipe_b {
                '|' | '7' | 'F' => true,
                _ => false,
            },
            Right => match pipe_b {
                '-' | '7' | 'J' => true,
                _ => false,
            },
            _ => false,
        },
        'J' => match dir {
            Up => match pipe_b {
                '|' | '7' | 'F' => true,
                _ => false,
            },
            Left => match pipe_b {
                '-' | 'F' | 'L' => true,
                _ => false,
            },
            _ => false,
        },
        '7' => match dir {
            Down => match pipe_b {
                '|' | 'J' | 'L' => true,
                _ => false,
            },
            Left => match pipe_b {
                '-' | 'F' | 'L' => true,
                _ => false,
            },
            _ => false,
        },
        'F' => match dir {
            Down => match pipe_b {
                '|' | 'J' | 'L' => true,
                _ => false,
            },
            Right => match pipe_b {
                '-' | '7' | 'J' => true,
                _ => false,
            },
            _ => false,

        },
        _ => false,

    }
}

impl PuzzleInput {
    fn get_directions(&self, x: isize, y: isize) -> Vec<(isize, isize)> {
        let mut directions = vec![];
        let curpipe = self.map[y as usize][x as usize];
        
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || nx >= self.map[0].len() as isize || ny < 0 || ny >= self.map.len() as isize {
                continue;
            }

            let nextpipe = self.map[ny as usize][nx as usize];
            if does_pipe_fit(curpipe, nextpipe, (dx, dy)) {
                directions.push((nx, ny));
            }
        }

        directions
    }

    fn resolve_s(&mut self) {
        for pipe in ['|', '-', 'L', 'J', '7', 'F'] {
            self.map[self.s_y as usize][self.s_x as usize] = pipe;
            if self.get_directions(self.s_x, self.s_y).len() == 2 {
                if DEBUG {
                    println!("S is a {} pipe", pipe);
                }
                break
            }
        }
    }

    fn raycast(&self, x: isize, y: isize, dir: (isize, isize), set: &mut HashSet<(isize, isize)>, set_inv: &mut HashSet<(isize, isize)>) {
        self._raycast(x, y, dir, set);
        self._raycast(x, y, (-dir.0, -dir.1), set_inv);
    }

    fn _raycast(&self, x: isize, y: isize, dir: (isize, isize), set: &mut HashSet<(isize, isize)>) {
        let pipe = self.map[y as usize][x as usize];
        let _dir = offset_to_dir(dir);
        match pipe {
            'F' => {
                if _dir == Direction::Up {
                    self.__raycast(x, y, (-1, 0), set); // Up
                    self.__raycast(x, y, (0, -1), set); // Left
                }
            },
            '7' => {
                if _dir == Direction::Right {
                    self.__raycast(x, y, (1, 0), set);  // Right
                    self.__raycast(x, y, (0, -1), set); // Up
                }
            },
            'J' => {
                if _dir == Direction::Down {
                    self.__raycast(x, y, (0, 1), set);   // Down
                    self.__raycast(x, y, (1, 0), set);   // Right
                }
            },
            'L' => {
                if _dir == Direction::Left {
                    self.__raycast(x, y, (-1, 0), set);  // Left
                    self.__raycast(x, y, (0, 1), set);   // Down
                }
            },
            '|' | '-' => {
                self.__raycast(x, y, dir, set);
            }
            _ => unreachable!(),
        }
    }

    fn __raycast(&self, x: isize, y: isize, dir: (isize, isize), set: &mut HashSet<(isize, isize)>) {
        let mut x = x;
        let mut y = y;
        loop {
            x += dir.0;
            y += dir.1;

            if x < 0 || x >= self.map[0].len() as isize || y < 0 || y >= self.map.len() as isize {
                break;
            }

            if self.map[y as usize][x as usize] != '.' {
                break;
            }

            set.insert((x, y));
        }
    }

    fn print_map(&self) {
        for row in &self.map {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
    }
}

pub fn part1(input: &PuzzleInput) -> isize {
    let mut input = input.clone();

    // Step 1: figure out what S should be.
    input.resolve_s();

    // Step 2: follow the pipes until we loop back to S.
    let mut x = input.s_x;
    let mut y = input.s_y;
    let mut visited = HashSet::new();
    visited.insert((x, y));

    let mut steps = 0;
    loop {
        let directions = input.get_directions(x, y);
        assert!(directions.len() == 2);
        let mut flag = false;
        for (nx, ny) in directions {
            if visited.contains(&(nx, ny)) {
                continue;
            }
            visited.insert((nx, ny));
            x = nx;
            y = ny;
            flag = true;
            break;
        }
        steps += 1;
        if !flag {
            break;
        }
    }
    (steps + 1) / 2
}

pub fn part2(input: &PuzzleInput) -> usize {
    let mut input = input.clone();

    // Step 1: figure out what S should be.
    input.resolve_s();

    // Step 2: follow the pipes until we loop back to S.
    //         We'll use this to make a clean copy of the map, without garbage.
    let mut x = input.s_x;
    let mut y = input.s_y;
    let mut visited = HashSet::new();
    visited.insert((x, y));

    loop {
        let directions = input.get_directions(x, y);
        assert!(directions.len() == 2);
        let mut flag = false;
        for (nx, ny) in directions {
            if visited.contains(&(nx, ny)) {
                continue;
            }
            visited.insert((nx, ny));
            x = nx;
            y = ny;
            flag = true;
            break;
        }
        if !flag {
            break;
        }
    }
    
    // Replace everything that's not in visited with '.'
    for y in 0..input.map.len() {
        for x in 0..input.map[0].len() {
            if !visited.contains(&(x as isize, y as isize)) {
                input.map[y][x] = '.';
            }
        }
    }

    if DEBUG {
        input.print_map();
    }


    // Step 3: Start a second walk from S, but this time raycast to find all areas that are on one side of a pipe.
    let mut x = input.s_x;
    let mut y = input.s_y;
    let mut visited = HashSet::new();
    let mut spotted = HashSet::new();
    let mut spotted2 = HashSet::new();
    visited.insert((x, y));

    loop {
        let directions = input.get_directions(x, y);
        assert!(directions.len() == 2);
        let mut flag = false;
        for (nx, ny) in directions {
            if visited.contains(&(nx, ny)) {
                continue;
            }
            visited.insert((nx, ny));
            match (nx - x, ny - y) {
                (1, 0) => {
                    // moved right, so raycast up
                    input.raycast(nx, ny, (0, -1), &mut spotted, &mut spotted2);
                },
                (-1, 0) => {
                    // moved left, so raycast down
                    input.raycast(nx, ny, (0, 1), &mut spotted, &mut spotted2);
                },
                (0, 1) => {
                    // moved down, so raycast right
                    input.raycast(nx, ny, (1, 0), &mut spotted, &mut spotted2);
                },
                (0, -1) => {
                    // moved up, so raycast left
                    input.raycast(nx, ny, (-1, 0), &mut spotted, &mut spotted2);
                },
                _ => unreachable!(),
            };
            x = nx;
            y = ny;
            flag = true;
            break;
        }
        if !flag {
            break;
        }
    }

    // overwrite everything in spotted with 'S'
    for (x, y) in &spotted {
        input.map[*y as usize][*x as usize] = 'o';
    }
    for (x, y) in &spotted2 {
        input.map[*y as usize][*x as usize] = 'I';
    }

    if DEBUG {
        println!();
        input.print_map();
    }

    let pipe_area = visited.len();
    let spotted_area = spotted.len();
    let spotted2_area = spotted2.len();

    if DEBUG {
        println!("Pipe area: {}", pipe_area);
        println!("Spotted area: {}", spotted_area);
        println!("Spotted2 area: {}", spotted2_area);
    }

    usize::min(spotted_area, spotted2_area)
}

pub fn read_input(filename: &str) -> PuzzleInput {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    let map: Vec<_> = reader
            .lines()
            .flatten()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

    let (s_x, s_y) = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == 'S' { Some((x as isize, y as isize)) } else { None })
        })
        .unwrap();

    PuzzleInput {
        map,
        s_x,
        s_y,
    }
}
