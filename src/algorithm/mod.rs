use crate::{graph::Graph, opp::OPP};
use infection::infection_set;

pub mod infection;
mod logger;

// compact way of tracking algorithm state.
pub struct Algorithm {
    pub orbit_reps: Vec<Vec<Vec<usize>>>,
    pub current_reps: Vec<Vec<usize>>,
    pub n: usize,
    infection_set: Vec<Vec<usize>>,
    pi: OPP, // input
    infected: usize,
}

impl Algorithm {
    pub fn print_current_state(&self) {
        logger::log(self);
    }

    pub fn init(graph: Graph, pi: OPP) -> Self {
        let n = graph.vertices.len();
        let current_reps: Vec<Vec<usize>> = vec![vec![0; n]];
        let orbit_reps: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; n]]]; // trivial colouring at level 0.
        let pi = pi; // GET THIS FROM SAUCY
        let infection_set = infection_set(&current_reps);
        let infected = 1;

        Self {
            orbit_reps,
            current_reps,
            n,
            pi,
            infection_set,
            infected,
        }
    }

    // MAIN_LOOP:
    pub fn run(&mut self) {
        while (self.infected as f32) < ((self.n / 2) as f32).floor() {
            self.run_level();
        }

        self.invert_colourings(); // swap 1s and 0s.
    }

    // EACH_LEVEL:
    pub fn run_level(&mut self) {
        self.current_reps.push(self.infection_set[0].clone());
        self.test_colourings(); // populate current_reps based off this.
        self.orbit_reps.push(self.current_reps.clone()); // add this levels reps to orbit_reps.
        self.infection_set = infection_set(&self.current_reps); // generate infection set for next level given pointer to current_reps
        self.infected += 1;
        self.reset_level();
    }

    fn reset_level(&mut self) {
        self.current_reps = Vec::new();
    }

    fn test_colourings(&mut self) {
        todo!() // WHOLE SEPARATE MODULE FOR THIS ALGORITHM.
    }

    fn invert_colourings(&mut self) {
        todo!()
    }
}

// #[cfg(test)]
// mod test {
//     use super::infection::infection_set;

//     #[test]
//     fn test_infection() {
//         let colouring = vec![0; 100];
//         let infection = infection_set(colouring);
//         for set in infection {
//             println!("orbit: {:?}", set);
//         }
//     }
// }
