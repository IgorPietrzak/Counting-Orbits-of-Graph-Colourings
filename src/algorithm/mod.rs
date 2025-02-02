use std::collections::HashMap;
use std::collections::HashSet;

use crate::opp::search_tree::get_permuations;
use crate::{graph::Graph, opp::OPP};
use infection::infection_set;

pub mod automorphism;
pub mod infection;
mod logger;
mod refinement;

// compact way of tracking algorithm state.
pub struct Algorithm {
    pub orbit_reps: Vec<Vec<usize>>,
    pub current_reps: Vec<Vec<usize>>,
    pub n: usize,
    infection_set: Vec<Vec<usize>>,
    pi: OPP, // input
    infected: usize,
    pub graph: Graph,
}

impl Algorithm {
    pub fn print_current_state(&self) {
        logger::quiet_log(self);
    }

    pub fn init(graph: Graph) -> Self {
        let n = graph.vertices.len();
        let current_reps: Vec<Vec<usize>> = vec![vec![0; n]];
        let orbit_reps: Vec<Vec<usize>> = vec![vec![0; n]]; // trivial colouring at level 0.
        let pi = graph.get_opp();
        let infection_set = infection_set(&current_reps);
        let infected = 1;

        Self {
            orbit_reps,
            current_reps,
            n,
            pi,
            infection_set,
            infected,
            graph,
        }
    }

    // MAIN_LOOP:
    pub fn run(&mut self) {
        while (self.infected as f32) < ((self.n / 2) as f32).floor() {
            self.run_level();
        }

        self.invert_colourings(); // swap 1s and 0s.
        self.print_current_state();
    }

    // EACH_LEVEL:
    pub fn run_level(&mut self) {
        self.print_current_state();
        self.test_colourings(); // populate current_reps based off this.
        self.orbit_reps.extend(self.current_reps.clone()); // add this levels reps to orbit_reps.
        self.reset_level();
    }

    fn reset_level(&mut self) {
        self.infection_set = infection_set(&self.current_reps);
        self.current_reps = Vec::new();
        self.infected += 1;
    }

    fn test_colourings(&mut self) {
        let current_reps = test_low_symmetry(&self.infection_set, &self.graph, &self.pi);
        self.current_reps = current_reps;
    }

    fn invert_colourings(&mut self) {
        let mut flipped_colours: Vec<Vec<usize>> = Vec::new();
        let mut seen_colours = HashSet::new();

        // Add the original colourings to the HashSet so we can check against them
        for row in self.orbit_reps.iter() {
            seen_colours.insert(row.clone()); // Insert original colourings into the HashSet
        }

        // Iterate over each row in orbit_reps and flip the values
        for row in self.orbit_reps.iter() {
            let flipped_row: Vec<usize> = row.iter().map(|&value| value ^ 1).collect(); // XOR flip

            // Check if the flipped colouring is already in orbit_reps
            if !seen_colours.contains(&flipped_row) {
                flipped_colours.push(flipped_row.clone()); // Add flipped row to the new collection
                seen_colours.insert(flipped_row); // Mark it as seen
            }
        }

        // Now extend self.orbit_reps with the flipped colourings
        self.orbit_reps.extend(flipped_colours);
    }
}

// for low symmetry graphs, it is faster to precompute Aut(G)
// gives orbit reps for current level.
fn test_low_symmetry(infection_set: &Vec<Vec<usize>>, graph: &Graph, opp: &OPP) -> Vec<Vec<usize>> {
    let automorphisms = get_permuations(&opp, &graph);
    // for each colouring find the canonical rep - this is the bucket (orbit) it falls into.
    let mut orbit_reps: Vec<Vec<usize>> = Vec::new();
    for colouring in infection_set {
        let canonical = get_canonical_form(&colouring, &automorphisms, &orbit_reps);
        if !(orbit_reps.contains(&canonical)) {
            orbit_reps.push(canonical);
        }
    }
    orbit_reps
}

// Get smallest colouring lexicographically.
fn get_canonical_form(
    colouring: &Vec<usize>,
    automorphisms: &Vec<Vec<usize>>,
    canonical_forms: &Vec<Vec<usize>>,
) -> Vec<usize> {
    let mut smallest = colouring.clone();
    for g in automorphisms.iter() {
        let curr = apply(g, &colouring);
        // if we stumble across a canonical rep we can stop the search.
        if canonical_forms.contains(&curr) {
            return curr;
        } else if curr < smallest {
            smallest = curr;
        }
    }

    smallest
}

pub fn apply(automorphism: &Vec<usize>, colouring: &Vec<usize>) -> Vec<usize> {
    // hashmap of where things go
    // <index,value>
    let mut perm_map: HashMap<usize, usize> = HashMap::new();
    for i in 0..automorphism.len() {
        // at the ith index we need the value of aut[i]
        perm_map.insert(i, colouring[automorphism[i]]);
    }
    let mut new_col = Vec::new();
    for i in 0..perm_map.len() {
        new_col.push(perm_map[&i]);
    }
    new_col
}

#[cfg(test)]
mod test {
    use super::apply;
    #[test]
    fn test_apply_aut_to_colouring() {
        let colouring = vec![0, 1, 0];
        let g = vec![1, 0, 2];
        let g1 = vec![0, 2, 1];
        assert_eq!(apply(&g, &colouring), vec![1, 0, 0]);
    }
}
