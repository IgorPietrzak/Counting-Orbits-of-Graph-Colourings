use infection::infection_set;

use crate::{graph::Graph, opp::OPP};
pub mod infection;

// compact way of tracking algorithm state.
#[derive(Debug)]
pub struct Algorithm {
    pub orbit_reps: Vec<Vec<Vec<usize>>>,
    pub current_reps: Vec<Vec<usize>>,
    pub n: usize,
    pi: OPP, // input
}

impl Algorithm {
    pub fn print_current_state(&self) {
        println!("====================================================================");
        println!("\n\n\n-----------------------------------------------------------");
        println!("CURRENT STATE");
        println!("-----------------------------------------------------------\n");
        println!("ORBIT REPS: \n");
        for i in self.orbit_reps.clone() {
            println!("* {:?}\n", i);
        }
        println!("-----------------------------------------------------------\n");
        println!("CURRENT REPS: \n");
        for i in self.current_reps.clone() {
            println!("* {:?}\n", i);
        }
        println!("-----------------------------------------------------------\n");
        println!("N: \n");
        println!("{:?}", self.n);
        println!("-----------------------------------------------------------\n");
        println!("PI");
        self.pi.print();
        println!("-----------------------------------------------------------");
        println!("\n\n\n====================================================================");
    }
    pub fn init(graph: Graph, pi: OPP) -> Self {
        let current_reps: Vec<Vec<usize>> = Vec::new();
        let n = graph.vertices.len();
        let orbit_reps: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; n]]]; // trivial colouring at level 0.
        let pi = pi;

        Self {
            orbit_reps,
            current_reps,
            n,
            pi,
        }
    }

    fn reset_level(&mut self) {
        self.current_reps = Vec::new();
    }

    pub fn run(&mut self) {
        // run saucy to get PI
        let infection_set = infection::infection_set(vec![0; self.n]);
        println!("is: {:?}", infection_set);
        self.current_reps.push(infection_set[0].clone());
        self.orbit_reps.push(self.current_reps.clone());
        self.reset_level();
        // run layers until floor(N/2)
    }
}

#[cfg(test)]
mod test {
    use super::infection::infection_set;
    use crate::opp::OPP;
    use std::collections::HashSet;

    #[test]
    fn test_infection() {
        let colouring = vec![0; 100];
        let infection = infection_set(colouring);
        for set in infection {
            println!("orbit: {:?}", set);
        }
    }
}
