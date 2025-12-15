use std::collections::{HashMap, HashSet};

use crate::common::LinesIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Start,
    Empty,
    Splitter,
    End,
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

    fn search_up_for_splitter(
        &self,
        pos: (usize, usize),
        mut seen: HashSet<(usize, usize)>,
    ) -> HashSet<(usize, usize)> {
        let mut out = HashSet::new();
        let (r, c) = pos;
        for i in (0..r).rev() {
            if matches!(self.grid[i][c], Node::Splitter | Node::Start) {
                break;
            }
            if (c > 0)
                && matches!(self.grid[i][c - 1], Node::Splitter | Node::Start)
            {
                out.insert((i, c - 1));
                seen.insert((i, c - 1));
            }
            if (c + 1 < self.m)
                && matches!(self.grid[i][c + 1], Node::Splitter | Node::Start)
            {
                out.insert((i, c + 1));
                seen.insert((i, c + 1));
            }
        }
        out
    }

    fn build_adj_2(&self) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {}

    fn build_adj(&self) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
        let mut out = HashMap::new();
        let mut to_add: HashSet<(usize, usize)> =
            (0..self.m).map(|c| (self.n, c)).collect();
        while !to_add.is_empty() {
            let mut add_next = HashSet::new();
            for pos in to_add.drain() {
                let mut pos_adj = HashSet::new();
                let (r, c) = pos;
                for i in (0..r).rev() {
                    if matches!(self.grid[i][c], Node::Splitter | Node::Start) {
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
        out
    }

    fn compute_num_paths(&self) -> usize {
        let adj = self.build_adj();
        assert!(adj.contains_key(&self.start));
        let num_paths_to: HashMap<(usize, usize), usize> = HashMap::new();

        let mut frontier = vec![self.start];

        while !frontier.is_empty() {
            frontier.entry()
        }

        num_paths_to
            .iter()
            .filter_map(|(k, v)| if k.0 + 1 == self.n { Some(v) } else { None })
            .sum()
    }
}

impl Manifold {}

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
    let mut mf = Manifold::from_grid(grid);
    format!("{:?}", mf.build_adj())
}
