use crate::bimap::*;
use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Clone, Default)]
pub struct Graph<T, W = ()> {
    verts: BiMap<T>,
    children: Vec<HashSet<(usize, W)>>,
    undirected: bool,
}

impl<T, W> Graph<T, W>
where
    T: Hash + Eq + Clone,
{
    #[must_use]
    fn new() -> Self {
        Self {
            verts: BiMap::new(),
            children: Vec::new(),
            undirected: false,
        }
    }

    // Makes a directed graph undirected by duplicating all of the edges
    fn undirect(&mut self)
    where
        W: Hash + Eq + Copy + Debug,
    {
        for (i, edge_list) in self.children.clone().into_iter().enumerate() {
            for (j, weight) in edge_list {
                self.children[j].insert((i, weight));
            }
        }
        self.undirected = true;
    }
}

pub fn num_reachable_targets<T, F1, F2>(src: T, is_tgt: F1, get_edges: F2) -> usize
where
    T: Eq + Hash + Copy + Debug,
    F1: Fn(T) -> bool,
    F2: Fn(T) -> HashSet<T>,
{
    let mut result = usize::from(is_tgt(src));

    // Stores visited nodes
    let mut visited: HashSet<T> = HashSet::new();
    visited.insert(src);

    // Stores nodes whose neighbors have not yet been checked
    let mut q: VecDeque<T> = VecDeque::new();
    q.push_back(src);

    // Pick the nearest vertex u that has been visited
    while let Some(u) = q.pop_front() {
        for nbr in get_edges(u) {
            // For each nbr of u that has not been visited...
            if visited.insert(nbr) {
                // Otherwise, mark nbr as visited and add it to the queue to
                // check its neighbors.
                q.push_back(nbr);

                // Count nbr if it is a target
                if is_tgt(nbr) {
                    result += 1;
                }
            }
        }
    }

    result
}

pub fn exists_path<T, F>(src: T, tgt: T, get_edges: F) -> bool
where
    T: Eq + Hash + Copy + Debug,
    F: Fn(T) -> HashSet<T>,
{
    if src == tgt {
        return true;
    }

    // Stores visited nodes
    let mut visited: HashSet<T> = HashSet::new();
    visited.insert(src);

    // Stores nodes whose neighbors have not yet been checked
    let mut q: VecDeque<T> = VecDeque::new();
    q.push_back(src);

    // Pick the nearest vertex u that has been visited
    while let Some(u) = q.pop_front() {
        for nbr in get_edges(u) {
            // For each nbr of u that has not been visited...
            if visited.insert(nbr) {
                // If nbr is the target, return true
                if nbr == tgt {
                    return true;
                }

                // Otherwise, mark nbr as visited and add it to the queue to
                // check its neighbors.
                q.push_back(nbr);
            }
        }
    }

    false
}

/// Takes in a `src: T`, a `tgt: T`, and a function
/// `get_edges: T -> HashSet<T>`.
///
/// Returns `None` if no path can be found from `src` to `tgt`, otherwise
/// returns a vector containing the vertices visited in a shortest path from
/// `src` to `tgt`.
pub fn shortest_path<T, F>(src: T, tgt: T, get_edges: F) -> Option<Vec<T>>
where
    T: Eq + Hash + Copy + Debug,
    F: Fn(T) -> HashSet<T>,
{
    if src == tgt {
        return Some(Vec::from([src]));
    }

    // Entries are (vertex, distance from source)
    let mut q: VecDeque<(T, usize)> = VecDeque::new();

    // Stores visited nodes, with a reference to their parent
    let mut visited: HashMap<T, Option<T>> = HashMap::new();

    // The src node has no parent, and is distance zero from itself.
    visited.insert(src, None);
    q.push_back((src, 0));

    // Pick the nearest vertex u that has been visited
    while let Some((u, dist)) = q.pop_front() {
        for nbr in get_edges(u) {
            if let Vacant(e) = visited.entry(nbr) {
                // For each neighbor nbr of u that has not been visited, mark
                // it as visited with parent u, and set its distance to u's
                // distance plus one.
                e.insert(Some(u));
                q.push_back((nbr, dist + 1));

                if nbr == tgt {
                    // If nbr is the target, then we create the output path.

                    // The number of vertices visited along the path is
                    // dist + 2, since dist is the number of steps from the src
                    // to the parent of the target.
                    let output_length = dist + 2;

                    // We will iterate over the path in reverse order, and
                    // assign the vertices to the output vector in reverse
                    // order. To do this, we will allocate the space we need
                    // for the output vector, and then fill it up
                    // back-to-front. This requires some unsafe code, since we
                    // will technically be accessing un-initialized memory as
                    // we fill the vector.
                    let mut output = Vec::with_capacity(output_length);

                    // This provides us a mutable reference to the
                    // un-initialized capacity of the output vector (right now,
                    // it is completely un-initialized)
                    let rem = output.spare_capacity_mut();

                    // To start, we will set the target as the last vertex
                    // visited in the path.
                    let mut cur = nbr;
                    rem[output_length - 1].write(cur);

                    // Now we iterate over the remaining steps we took, filling
                    // up the output vector;
                    let mut i = 1;
                    while let Some(parent) = visited[&cur] {
                        i += 1;
                        cur = parent;
                        rem[output_length - i].write(cur);
                    }

                    // We want to make sure that we reached 0.
                    // assert_eq!(output_length - i, 0);

                    unsafe {
                        output.set_len(output_length);
                    }

                    return Some(output);
                }
            }
        }
    }

    None
}

pub fn num_of_paths<T, F1, F2>(src: T, is_tgt: &F1, get_edges: &F2) -> usize
where
    T: Eq + Hash + Debug + Copy,
    F1: Fn(T) -> bool,
    F2: Fn(T) -> HashSet<T>,
{
    // Assuming src != tgt
    // assert_ne!(src, tgt);

    let mut count = 0;
    for nbr in get_edges(src) {
        if is_tgt(nbr) {
            count += 1;
        } else {
            count += num_of_paths(nbr, is_tgt, get_edges);
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_paths() {
        // 0 1 3
        // 2   5
        // 4 6 7 8
        //     9
        let get_edges = |x: u8| match x {
            0 => HashSet::from([1, 2]),
            1 => HashSet::from([3]),
            2 => HashSet::from([4]),
            3 => HashSet::from([5]),
            4 => HashSet::from([6]),
            5 | 6 => HashSet::from([7]),
            7 => HashSet::from([8, 9]),
            _ => HashSet::new(),
        };

        let is_tgt = |x: u8| x >= 8;

        assert_eq!(num_of_paths(0, &is_tgt, &get_edges), 4);
    }

    #[test]
    fn test_num_reachable_targets() {
        let get_edges = |x: u8| {
            if x == 0 {
                HashSet::from([1, u8::MAX])
            } else if x < u8::MAX {
                HashSet::from([x - 1, x + 1])
            } else if x == u8::MAX {
                HashSet::from([0, u8::MAX - 1])
            } else {
                HashSet::new()
            }
        };

        let is_prime = |x: u8| {
            if x <= 3 {
                x >= 2
            } else if x % 2 == 0 {
                false
            } else {
                let mut div = 3;
                while div < x {
                    if x % div == 0 {
                        return false;
                    }
                    div += 2;
                }
                true
            }
        };

        let num_primes = (0..=u8::MAX).map(is_prime).filter(|&b| b).count();

        for n in 0..u8::MAX {
            assert_eq!(num_reachable_targets(n, is_prime, get_edges), num_primes);
        }
    }

    #[test]
    fn test_exists_path() {
        let get_edges = |x: u8| match x {
            0 => HashSet::from([1, 5]),
            1 => HashSet::from([0, 2]),
            2 => HashSet::from([1, 3]),
            3 => HashSet::from([2, 4]),
            4 => HashSet::from([3, 5]),
            5 => HashSet::from([4, 0]),
            _ => HashSet::new(),
        };

        assert!(exists_path(0, 5, get_edges));
        assert!(exists_path(0, 4, get_edges));
        assert!(exists_path(0, 2, get_edges));
        assert!(exists_path(0, 0, get_edges));
    }

    #[test]
    fn test_shortest_path_two_paths() {
        let get_edges = |x: u8| match x {
            0 => HashSet::from([1, 5]),
            1 => HashSet::from([0, 2]),
            2 => HashSet::from([1, 3]),
            3 => HashSet::from([2, 4]),
            4 => HashSet::from([3, 5]),
            5 => HashSet::from([4, 0]),
            _ => HashSet::new(),
        };

        assert_eq!(shortest_path(0, 5, get_edges), Some(Vec::from([0, 5])));
        assert_eq!(shortest_path(0, 4, get_edges), Some(Vec::from([0, 5, 4])));
        assert_eq!(shortest_path(0, 2, get_edges), Some(Vec::from([0, 1, 2])));
        assert_eq!(shortest_path(0, 0, get_edges), Some(Vec::from([0])));
    }

    #[test]
    fn test_shortest_path_no_path() {
        let output = shortest_path(0, 1, |_| HashSet::new());
        assert_eq!(output, None);
    }

    #[test]
    fn test_graph() {
        let g: Graph<&str, f64> = Graph::default();
        assert!(!g.undirected);
    }
}