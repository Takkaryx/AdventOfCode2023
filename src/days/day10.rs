use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::fs::read_to_string;

use anyhow::{anyhow, Result};
use ndarray::Array2;
use petgraph::graphmap::UnGraphMap;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Vert,
    Horiz,
    NE,
    NW,
    SW,
    SE,
    Origin,

    #[default]
    Empty,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Filled {
    Loop,
    Filled,

    #[default]
    Empty,
}

impl TryFrom<char> for Pipe {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::Vert),
            '-' => Ok(Self::Horiz),
            'L' => Ok(Self::NE),
            'J' => Ok(Self::NW),
            '7' => Ok(Self::SW),
            'F' => Ok(Self::SE),
            'S' => Ok(Self::Origin),
            '.' => Ok(Self::Empty),
            _ => Err(anyhow!("Invalid pipe")),
        }
    }
}

impl Pipe {
    fn get_pos_up(&self) -> Vec<Pipe> {
        match self {
            Pipe::Vert => vec![Pipe::Origin, Pipe::Vert, Pipe::SE, Pipe::SW],
            Pipe::NE => vec![Pipe::Origin, Pipe::Vert, Pipe::SE, Pipe::SW],
            Pipe::NW => vec![Pipe::Origin, Pipe::Vert, Pipe::SE, Pipe::SW],
            Pipe::Origin => vec![Pipe::Vert, Pipe::SE, Pipe::SW],
            _ => vec![],
        }
    }

    fn get_pos_down(&self) -> Vec<Pipe> {
        match self {
            Pipe::Vert => vec![Pipe::Origin, Pipe::Vert, Pipe::NE, Pipe::NW],
            Pipe::SE => vec![Pipe::Origin, Pipe::Vert, Pipe::NE, Pipe::NW],
            Pipe::SW => vec![Pipe::Origin, Pipe::Vert, Pipe::NE, Pipe::NW],
            Pipe::Origin => vec![Pipe::Vert, Pipe::NE, Pipe::NW],
            _ => vec![],
        }
    }

    fn get_pos_right(&self) -> Vec<Pipe> {
        match self {
            Pipe::Horiz => vec![Pipe::Origin, Pipe::Horiz, Pipe::SW, Pipe::NW],
            Pipe::SE => vec![Pipe::Origin, Pipe::Horiz, Pipe::SW, Pipe::NW],
            Pipe::NE => vec![Pipe::Origin, Pipe::Horiz, Pipe::SW, Pipe::NW],
            Pipe::Origin => vec![Pipe::Horiz, Pipe::SW, Pipe::NW],
            _ => vec![],
        }
    }

    fn get_pos_left(&self) -> Vec<Pipe> {
        match self {
            Pipe::Horiz => vec![Pipe::Origin, Pipe::Horiz, Pipe::SE, Pipe::NE],
            Pipe::SW => vec![Pipe::Origin, Pipe::Horiz, Pipe::SE, Pipe::NE],
            Pipe::NW => vec![Pipe::Origin, Pipe::Horiz, Pipe::SE, Pipe::NE],
            Pipe::Origin => vec![Pipe::Horiz, Pipe::SE, Pipe::NE],
            _ => vec![],
        }
    }
    fn get_scaled_up_version(&self) -> Array2<Pipe> {
        match self {
            Pipe::Vert => Array2::from_shape_vec(
                (3, 3),
                vec![
                    Pipe::Empty,
                    Pipe::Vert,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Vert,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Vert,
                    Pipe::Empty,
                ],
            )
            .expect("Should have correct shape"),
            Pipe::Horiz => Array2::from_shape_vec(
                (3, 3),
                vec![
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Horiz,
                    Pipe::Horiz,
                    Pipe::Horiz,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Empty,
                ],
            )
            .expect("Should have correct shape"),
            Pipe::NE => Array2::from_shape_vec(
                (3, 3),
                vec![
                    Pipe::Empty,
                    Pipe::Vert,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::NE,
                    Pipe::Horiz,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Empty,
                ],
            )
            .expect("Should have correct shape"),
            Pipe::NW => Array2::from_shape_vec(
                (3, 3),
                vec![
                    Pipe::Empty,
                    Pipe::Vert,
                    Pipe::Empty,
                    Pipe::Horiz,
                    Pipe::NW,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Empty,
                ],
            )
            .expect("Should have correct shape"),
            Pipe::SW => Array2::from_shape_vec(
                (3, 3),
                vec![
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Horiz,
                    Pipe::SW,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Vert,
                    Pipe::Empty,
                ],
            )
            .expect("Should have correct shape"),
            Pipe::SE => Array2::from_shape_vec(
                (3, 3),
                vec![
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::SE,
                    Pipe::Horiz,
                    Pipe::Empty,
                    Pipe::Vert,
                    Pipe::Empty,
                ],
            )
            .expect("Should have correct shape"),
            Pipe::Empty => Array2::from_shape_vec(
                (3, 3),
                vec![
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Empty,
                    Pipe::Empty,
                ],
            )
            .expect("Should have correct shape"),
            Pipe::Origin => Array2::from_shape_vec(
                (3, 3),
                vec![
                    Pipe::Empty,
                    Pipe::Vert,
                    Pipe::Empty,
                    Pipe::Horiz,
                    Pipe::Origin,
                    Pipe::Horiz,
                    Pipe::Empty,
                    Pipe::Vert,
                    Pipe::Empty,
                ],
            )
            .expect("Should have correct shape"),
        }
    }
}

struct Grid {
    origin: (usize, usize),
    matrix: Array2<Pipe>,
}

impl Grid {
    fn get_graph(&self) -> UnGraphMap<(usize, usize), ()> {
        let mut graph = UnGraphMap::new();

        for ((i, j), p) in self.matrix.indexed_iter() {
            if i > 0 && p.get_pos_up().contains(&self.matrix[(i - 1, j)]) {
                graph.add_edge((i, j), (i - 1, j), ());
            }
            if i < self.matrix.nrows() - 1 && p.get_pos_down().contains(&self.matrix[(i + 1, j)]) {
                graph.add_edge((i, j), (i + 1, j), ());
            }

            if j > 0 && p.get_pos_left().contains(&self.matrix[(i, j - 1)]) {
                graph.add_edge((i, j), (i, j - 1), ());
            }
            if j < self.matrix.ncols() - 1 && p.get_pos_right().contains(&self.matrix[(i, j + 1)]) {
                graph.add_edge((i, j), (i, j + 1), ());
            }
        }
        graph
    }
}

fn find_loop(
    graph: &UnGraphMap<(usize, usize), ()>,
    beginning: (usize, usize),
) -> Result<Vec<(usize, usize)>> {
    let mut queue = vec![vec![beginning]];

    while let Some(next) = queue.pop() {
        let last = *next.last().expect("Paths should not be empty");

        if last == beginning && next.len() > 1 {
            return Ok(next);
        }

        let neighbors: Vec<(usize, usize)> = graph
            .neighbors(last)
            .filter(|n| {
                if next.len() < 2 {
                    return true;
                }
                if let Some(x) = next.get(next.len() - 2) {
                    n != x
                } else {
                    true
                }
            })
            .collect();

        for neighbor in neighbors {
            let mut path = next.clone();
            path.push(neighbor);

            queue.push(path);
        }
    }

    Err(anyhow!("ERROR: Loop not found"))
}

fn parse_input(input: &str) -> Result<Grid> {
    let lines: Vec<&str> = input.lines().collect();

    let num_rows = lines.len();
    let num_colm = lines[0].len();

    let mut matrix: Array2<Pipe> = Array2::default((num_rows, num_colm));

    let mut origin = (0, 0);
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            matrix[(i, j)] = Pipe::try_from(c)?;

            if matches!(matrix[(i, j)], Pipe::Origin) {
                origin = (i, j);
            }
        }
    }

    Ok(Grid { origin, matrix })
}

fn scale_up(matrix: &Array2<Pipe>) -> Array2<Pipe> {
    let n_rows = matrix.nrows();
    let n_cols = matrix.ncols();

    let mut new_array: Array2<Pipe> = Array2::default((3 * n_rows, 3 * n_cols));

    for ((i, j), p) in matrix.indexed_iter() {
        let scaled_up = p.get_scaled_up_version();

        for h1 in [3 * i, 3 * i + 1, 3 * i + 2] {
            for h2 in [3 * j, 3 * j + 1, 3 * j + 2] {
                new_array[(h1, h2)] = scaled_up[(h1 - 3 * i, h2 - 3 * j)]
            }
        }
    }

    new_array
}

fn scale_down<T>(matrix: &Array2<T>) -> Array2<T>
where
    T: Clone + Copy + Default,
{
    let n_rows = matrix.nrows() / 3;
    let n_cols = matrix.ncols() / 3;

    let mut new_array = Array2::default((n_rows, n_cols));

    for i in 0..n_rows {
        for j in 0..n_cols {
            new_array[(i, j)] = matrix[(3 * i + 1, 3 * j + 1)]
        }
    }

    new_array
}

fn transform_matrix(matrix: &Array2<Pipe>, lp: &[(usize, usize)]) -> Array2<Filled> {
    let mut new_matrix: Array2<Filled> = Array2::default((matrix.nrows(), matrix.ncols()));

    for (i, j) in lp {
        for h1 in [3 * i, 3 * i + 1, 3 * i + 2] {
            for h2 in [3 * j, 3 * j + 1, 3 * j + 2] {
                if !matches!(matrix[(h1, h2)], Pipe::Empty) {
                    new_matrix[(h1, h2)] = Filled::Loop
                }
            }
        }
    }

    new_matrix
}

fn flood_fill(matrix: &Array2<Filled>) -> Array2<Filled> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: Vec<(usize, usize)> = vec![(0, 0)];

    let mut new_array: Array2<Filled> = Array2::default((matrix.nrows(), matrix.ncols()));

    for ((i, j), v) in matrix.indexed_iter() {
        if let Filled::Loop = v {
            new_array[(i, j)] = *v
        }
    }

    while let Some(next) = queue.pop() {
        visited.insert(next);
        new_array[next] = Filled::Filled;

        let (i, j) = next;

        if i > 0 && !matches!(matrix[(i - 1, j)], Filled::Loop) && !visited.contains(&(i - 1, j)) {
            queue.push((i - 1, j))
        }
        if i < matrix.nrows() - 1
            && !matches!(matrix[(i + 1, j)], Filled::Loop)
            && !visited.contains(&(i + 1, j))
        {
            queue.push((i + 1, j))
        }
        if j > 0 && !matches!(matrix[(i, j - 1)], Filled::Loop) && !visited.contains(&(i, j - 1)) {
            queue.push((i, j - 1))
        }
        if j < matrix.ncols() - 1
            && !matches!(matrix[(i, j + 1)], Filled::Loop)
            && !visited.contains(&(i, j + 1))
        {
            queue.push((i, j + 1))
        }
    }

    new_array
}

///////////////////////////////////////////////////////////////////////////////

fn solution1(map: &Grid) -> u64 {
    let lp = find_loop(&map.get_graph(), map.origin).unwrap();

    (lp.len() as u64 - 1) / 2 // Subtract one to remove the origin.
}

fn solution2(map: &Grid) -> u64 {
    let lp = find_loop(&map.get_graph(), map.origin).unwrap();

    let scaled_up = scale_up(&map.matrix);
    let transformed_matrix = transform_matrix(&scaled_up, &lp);

    let filled = flood_fill(&transformed_matrix);
    let scaled_down = scale_down(&filled);

    // println!("{:?}", scaled_down);

    scaled_down
        .iter()
        .filter(|e| matches!(e, Filled::Empty))
        .count() as u64
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let contents = read_to_string("input/day10_input.txt").unwrap();
    let map = parse_input(&contents).unwrap();

    let sol1: u64 = solution1(&map);
    let sol2: u64 = solution2(&map);

    (Solution::from(sol1), Solution::from(sol2))
}
