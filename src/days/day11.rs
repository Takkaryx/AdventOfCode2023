use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

use anyhow::{anyhow, Result};
use ndarray::{Array2, Axis};
// use petgraph::graphmap::UnGraphMap;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Galaxy,

    #[default]
    Err,
}

impl TryFrom<char> for Space {
    type Error = anyhow::Error;
    fn try_from(val: char) -> Result<Self, Self::Error> {
        match val {
            '#' => Ok(Self::Galaxy),
            '.' => Ok(Self::Empty),
            _ => Err(anyhow!("invalid value found in grid")),
        }
    }
}

#[derive(Clone)]
struct Sky {
    galaxies: Vec<(usize, usize)>,
    matrix: Array2<Space>,
}

impl Sky {
    fn expand(&mut self, num_added: usize) {
        let mut matrix_copy = self.matrix.to_owned();
        let mut row_locs: Vec<usize> = vec![];
        let mut col_locs: Vec<usize> = vec![];
        for i in 0..self.matrix.nrows() {
            if self
                .matrix
                .row(i)
                .clone()
                .into_iter()
                .all(|val| val == &Space::Empty)
            {
                row_locs.push(i);
            }
        }
        for j in 0..self.matrix.ncols() {
            if self
                .matrix
                .column(j)
                .clone()
                .into_iter()
                .all(|val| val == &Space::Empty)
            {
                col_locs.push(j);
            }
        }
        let new_row = vec![Space::Empty; self.matrix.ncols()];
        let new_row_array = Array2::from_shape_vec((1, new_row.len()), new_row).unwrap();
        for index in row_locs {
            matrix_copy
                .insert_axis(Axis(0))
                .axis_iter_mut(Axis(0))
                .into_iter()
                .enumerate()
                .for_each(|(i, mut row)| {
                    if i == index {
                        row.assign(&new_row_array);
                    }
                })
        }
    }
}

fn parse_input(input: &str) -> Result<Sky> {
    let lines: Vec<&str> = input.lines().collect();
    let num_rows = lines.len();
    let num_colm = lines[0].len();

    let mut matrix: Array2<Space> = Array2::default((num_rows, num_colm));
    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let point = Space::try_from(c)?;
            matrix[(i, j)] = point;
            if point == Space::Galaxy {
                galaxies.push((i, j));
            }
        }
    }
    Ok(Sky { galaxies, matrix })
}

///////////////////////////////////////////////////////////////////////////////

fn sol1(sky: &mut Sky) -> u64 {
    sky.expand(1);
    0
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let contents = read_to_string("input/day11_example.txt").unwrap();
    let graph = parse_input(&contents).unwrap();
    let sol1: u64 = sol1(&mut graph.clone());
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
