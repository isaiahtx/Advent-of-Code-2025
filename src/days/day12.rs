use crate::direction::{Coords, Direction};
use crate::uptree::UpTree;
use crate::utils::{lines_to_grid_of_chars, LinesIterator};
use std::collections::{HashSet, VecDeque};

type Nbr = (Direction, Coords);
type NbrsGrid = Vec<Vec<Nbrs>>;
type RegionsList = Vec<Vec<Coords>>;

#[derive(Debug, Clone)]
struct Nbrs {
    // [north,northeast,east,southeast,south,southwest,west,northwest]
    nbrs: [Option<Coords>; 8],
    count: usize,
}

impl Nbrs {
    const fn new() -> Self {
        Self {
            nbrs: [None; 8],
            count: 0,
        }
    }

    const fn get(&self, dir: Direction) -> Option<Coords> {
        self.nbrs[dir.to_num()]
    }

    fn add(&mut self, dir: Direction, coords: Coords) -> bool {
        let output = self.nbrs[dir.to_num()].is_none();
        self.nbrs[dir.to_num()] = Some(coords);
        if output {
            self.count += 1;
        }
        output
    }

    fn num_cardinals(&self) -> usize {
        usize::from(self.nbrs[0].is_some())
            + usize::from(self.nbrs[2].is_some())
            + usize::from(self.nbrs[4].is_some())
            + usize::from(self.nbrs[6].is_some())
    }

    fn get_cardinals(&self) -> Vec<Nbr> {
        let mut output = Vec::new();
        for i in [0, 2, 4, 6] {
            if let Some(coords) = self.nbrs[i] {
                output.push((Direction::from(i).unwrap(), coords));
            }
        }
        output
    }

    fn missing_cardinals(&self) -> Vec<Direction> {
        let mut output = Vec::new();
        for i in [0, 2, 4, 6] {
            if self.nbrs[i].is_none() {
                output.push(Direction::from(i).unwrap());
            }
        }
        output
    }
}

fn get_components(grid: &[Vec<char>]) -> (NbrsGrid, RegionsList) {
    let height = grid.len();
    let width = grid[0].len();

    let mut ut: UpTree<Coords> = UpTree::new();
    let mut q = VecDeque::new();
    let mut seen: HashSet<Coords> = HashSet::new();
    let mut filtered_nbrs_grid: NbrsGrid = Vec::with_capacity(height);
    for (i, row) in grid.iter().enumerate() {
        filtered_nbrs_grid.push(Vec::with_capacity(width));
        for (j, cur) in row.iter().enumerate() {
            filtered_nbrs_grid[i].push(Nbrs::new());
            q.push_back((i, j));
            ut.insert_root((i, j));

            let cur = *cur;
            let mut north = false;
            let mut south = false;

            if i > 0 {
                north = true;
                if grid[i - 1][j] == cur {
                    filtered_nbrs_grid[i][j].add(Direction::N, (i - 1, j));
                }
            }

            if i + 1 < height {
                south = true;
                if grid[i + 1][j] == cur {
                    filtered_nbrs_grid[i][j].add(Direction::S, (i + 1, j));
                }
            }

            if j + 1 < width {
                if grid[i][j + 1] == cur {
                    filtered_nbrs_grid[i][j].add(Direction::E, (i, j + 1));
                }
                if north && grid[i - 1][j + 1] == cur {
                    filtered_nbrs_grid[i][j].add(Direction::NE, (i - 1, j + 1));
                }
                if south && grid[i + 1][j + 1] == cur {
                    filtered_nbrs_grid[i][j].add(Direction::SE, (i + 1, j + 1));
                }
            }

            if j > 0 {
                if grid[i][j - 1] == cur {
                    filtered_nbrs_grid[i][j].add(Direction::W, (i, j - 1));
                }
                if north && grid[i - 1][j - 1] == cur {
                    filtered_nbrs_grid[i][j].add(Direction::NW, (i - 1, j - 1));
                }
                if south && grid[i + 1][j - 1] == cur {
                    filtered_nbrs_grid[i][j].add(Direction::SW, (i + 1, j - 1));
                }
            }
        }
    }

    while !q.is_empty() {
        let v = q.pop_back().unwrap();
        for (_, w) in filtered_nbrs_grid[v.0][v.1].get_cardinals() {
            if !seen.contains(&w) {
                ut.union(&w, &v);
            }
        }
        seen.insert(v);
    }

    (
        filtered_nbrs_grid,
        ut.flatten()
            .iter()
            .map(|component| component.iter().map(|(&coords, ())| coords).collect())
            .collect(),
    )
}

/// # Panics
///
/// stfu
pub fn run1(lines: &mut LinesIterator) -> String {
    let grid: Vec<_> = lines_to_grid_of_chars(lines).collect();
    let (filtered_nbrs_grid, regions) = get_components(&grid);

    let mut output = 0;

    for region in regions {
        let area = region.len();
        let mut perimeter = 0;
        for v in region {
            perimeter += 4 - filtered_nbrs_grid[v.0][v.1].num_cardinals();
        }
        output += area * perimeter;
    }

    format!("{output}")
}

/// # Panics
///
/// stfu
pub fn run2(lines: &mut LinesIterator) -> String {
    let grid: Vec<_> = lines_to_grid_of_chars(lines).collect();

    let (nbrs, regions) = get_components(&grid);
    let mut output = 0;

    for region in regions {
        let area = region.len();
        let mut edge_nodes: HashSet<(Coords, Direction)> = region
            .iter()
            .flat_map(|&v| {
                nbrs[v.0][v.1]
                    .missing_cardinals()
                    .iter()
                    .map(|&d| (v, d))
                    .collect::<Vec<(Coords, Direction)>>()
            })
            .collect();

        let mut num_edges = 0;

        while !edge_nodes.is_empty() {
            let (start, start_edge_facing) = *edge_nodes.iter().next().unwrap();
            let start_travelling_dir = start_edge_facing.turn_right();

            edge_nodes.remove(&(start, start_edge_facing));
            num_edges += 1;

            let mut cur = start;
            let mut edge_facing = start_edge_facing;
            let mut travelling_dir = start_travelling_dir;

            loop {
                let fwd_corner = edge_facing.combine_cardinal(travelling_dir).unwrap();

                if let Some(new) = nbrs[cur.0][cur.1].get(fwd_corner) {
                    if region.contains(&new) {
                        std::mem::swap(&mut edge_facing, &mut travelling_dir);
                        edge_facing = edge_facing.reflect();
                        cur = new;
                        edge_nodes.remove(&(cur, edge_facing));
                        if (cur, edge_facing) == (start, start_edge_facing) {
                            break;
                        }
                        num_edges += 1;
                        continue;
                    }
                }

                if let Some(new) = nbrs[cur.0][cur.1].get(travelling_dir) {
                    cur = new;
                    edge_nodes.remove(&(cur, edge_facing));
                    if (cur, edge_facing) == (start, start_edge_facing) {
                        num_edges -= 1;
                        break;
                    }
                    continue;
                }

                edge_facing = edge_facing.reflect();

                std::mem::swap(&mut edge_facing, &mut travelling_dir);
                edge_nodes.remove(&(cur, edge_facing));
                if (cur, edge_facing) == (start, start_edge_facing) {
                    break;
                }
                num_edges += 1;
            }
        }

        output += area * num_edges;
    }

    format!("{output}")
}
