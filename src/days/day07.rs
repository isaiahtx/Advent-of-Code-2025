use std::collections::{BTreeMap, HashMap, HashSet};

use crate::common::LinesIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Start,
    Empty,
    Splitter,
}

impl Node {
    const fn from_char(c: char) -> Self {
        match c {
            'S' => Self::Start,
            '^' => Self::Splitter,
            _ => Self::Empty,
        }
    }
}

#[derive(Debug, Clone)]
struct Manifold {
    grid: Vec<Vec<Node>>,
    start: (usize, usize),
    n: usize,
    m: usize,
    num_splits: usize,
}

impl Manifold {
    fn _find_start(grid: &[Vec<Node>]) -> (usize, usize) {
        grid.iter()
            .enumerate()
            .find_map(|(row_i, row)| {
                row.iter()
                    .position(|&n| n == Node::Start)
                    .map(|col_i| (row_i, col_i))
            })
            .unwrap()
    }

    fn from_grid(grid: Vec<Vec<Node>>) -> Self {
        let start = Self::_find_start(&grid);
        let n = grid.len();
        let m = grid[0].len();
        Self {
            grid,
            start,
            n,
            m,
            num_splits: 0,
        }
    }

    fn get_beam_nbrs(&mut self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (r, c) = pos;
        if r + 1 == self.n {
            return vec![];
        }
        if self.grid[r + 1][c] != Node::Splitter {
            return vec![(r + 1, c)];
        }
        self.num_splits += 1;
        let mut out = vec![];
        if c > 0 {
            out.push((r + 1, c - 1));
        }
        if c + 1 < self.m {
            out.push((r + 1, c + 1));
        }
        out
    }

    fn update(
        &mut self,
        mut beams: HashSet<(usize, usize)>,
    ) -> HashSet<(usize, usize)> {
        let mut new_beams = HashSet::new();

        for beam in beams.drain() {
            let next_beams = self.get_beam_nbrs(beam);
            new_beams.extend(next_beams);
        }

        new_beams
    }

    fn run1(&mut self) {
        let mut beams = HashSet::from([self.start]);
        while !beams.is_empty() {
            beams = self.update(beams);
        }
    }

    fn build_adj(&self) -> BTreeMap<(usize, usize), HashSet<(usize, usize)>> {
        let mut out = BTreeMap::new();
        let ends: HashSet<(usize, usize)> =
            (0..self.m).map(|c| (self.n, c)).collect();
        let mut to_add: HashSet<(usize, usize)> = ends.clone();
        while !to_add.is_empty() {
            let mut add_next = HashSet::new();
            for pos in to_add.drain() {
                let mut pos_adj = HashSet::new();
                let (r, c) = pos;
                for i in (0..r).rev() {
                    if matches!(self.grid[i][c], Node::Splitter) {
                        break;
                    }
                    if matches!(self.grid[i][c], Node::Start) {
                        let new_pos = (i, c);
                        pos_adj.insert(new_pos);
                        if !out.contains_key(&new_pos) {
                            add_next.insert(new_pos);
                        }
                        break;
                    }
                    if (c > 0) && matches!(self.grid[i][c - 1], Node::Splitter)
                    {
                        let new_pos = (i, c - 1);
                        pos_adj.insert(new_pos);
                        if !out.contains_key(&new_pos) {
                            add_next.insert(new_pos);
                        }
                    }
                    if (c + 1 < self.m)
                        && matches!(self.grid[i][c + 1], Node::Splitter)
                    {
                        let new_pos = (i, c + 1);
                        pos_adj.insert(new_pos);
                        if !out.contains_key(&new_pos) {
                            add_next.insert(new_pos);
                        }
                    }
                }
                out.insert(pos, pos_adj);
            }
            to_add = add_next;
        }
        out.insert((self.n + 1, 0), ends);
        out
    }

    fn compute_num_paths(&self) -> usize {
        let mut adj = self.build_adj();
        let mut num_paths_to: HashMap<(usize, usize), usize> = HashMap::new();
        while let Some((pos, nbrs)) = adj.pop_first() {
            if pos == self.start {
                num_paths_to.insert(pos, 1);
            } else {
                let mut num_paths: usize = 0;
                for nbr in nbrs {
                    if let Some(to_add) = num_paths_to.get(&nbr) {
                        num_paths += to_add;
                    }
                }
                num_paths_to.insert(pos, num_paths);
            }
        }
        *num_paths_to.get(&(self.n + 1, 0)).unwrap()
    }
}

fn parse_input(lines: &mut LinesIterator) -> Vec<Vec<Node>> {
    lines
        .map(|line| line.unwrap().chars().map(Node::from_char).collect())
        .collect()
}

pub fn run1(lines: &mut LinesIterator) -> String {
    let grid = parse_input(lines);
    let mut mf = Manifold::from_grid(grid);
    mf.run1();
    format!("{}", mf.num_splits)
}

pub fn run2(lines: &mut LinesIterator) -> String {
    let grid = parse_input(lines);
    let mf = Manifold::from_grid(grid);
    format!("{:?}", mf.compute_num_paths())
}
