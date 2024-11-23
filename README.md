# Code explanation.

## What does it do?

This algorithm is counting orbit representatives of binary colourings of graphs. Let `G` be a graph. We say a colouring `A` is in the orbit of `B` if there is a `g` in the Automorphism group `Aut(G)`
of `G` such that `g.A = B`. The algorithm collects 1 element from each orbit. This element is known as the orbit representative. Once it finds all orbit representatives
it knows how many orbits there are.

## How does it work?

We represent colourings as vectors. Since we are considering binary colourings the vectors will only consist of 1s and 0s. For example `[0,1,0]` tells us that 
vertex 0  has colour 0, vertex 1 has colour 1 and vertex 2 has colour 0. When a vertex has colour 1 we say it is infected.

The idea is to start with i = 1 infected vertices and look at all the possible colourings. The algorithm checks which colourings are in the same orbits and collects
the orbit representatives for i = 1 infected vertices. It then infects the orbit representatives found at the previous iteration and again collects the orbit representatives.


It carries on doing this until we reach i = floor(n/2) infected vertices for a graph of n vertices. The last step is to invert all the colourings (flip the 1s to 0s and vice versa)
and add them to the found orbit representatives.

## How is it implemented?

The main component of the algorithm is the Algorithm struct.

```rust
pub struct Algorithm {
    pub orbit_reps: Vec<Vec<Vec<usize>>>, // output of the algorithm
    pub current_reps: Vec<Vec<usize>>, // representatives for a specific number of infected vertices.
    pub n: usize, // number of vertices of the graph
    infection_set: Vec<Vec<usize>>, // infected orbit reps from previous iteration
    pi: OPP, // will be used in finding automorphisms
    infected: usize, // number of currently infected vertices.
}
```

We use the fields of this struct to keep track of the current state of the algorithm.


We then have functions acting on these variables which is how we execute the algorithm:

```rust
impl Algorithm {
    pub fn print_current_state(&self) {
        logger::log(self);
    }

// STARTING VALUES
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
        self.test_colourings(); // populate current_reps based off this. - This is checking which colourings are in the same orbit
        self.orbit_reps.push(self.current_reps.clone()); // add this level's reps to orbit_reps.
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
```

### How do we infect?

In the 2nd section we mentioned infecting the previous iteration's orbit reps. This is done by switching a 0 to a 1 in all possible ways. For example the infection set of `[0,1,0]` is `[[1,1,0],[0,1,1]]`.
This is done by the infection submodule:

```rust
pub fn infection_set(current_reps: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut infection_set = Vec::new();
    for colouring in current_reps {
        infection_set.extend(build_infection(colouring));
    }
    infection_set
}

fn build_infection(colouring: &Vec<usize>) -> Vec<Vec<usize>> {
    let parallel_colourings: Vec<Vec<usize>> = (0..colouring.len())
        .into_par_iter() // creates parallel iterator.
        .filter_map(|i| {
            if colouring[i] == 0 {
                let mut new_colouring = colouring.clone(); // We only clone when we need to flip 0 to 1.
                new_colouring[i] = 1;
                Some(new_colouring)
            } else {
                None // Do nothing when encounter a 1.
            }
        })
        .collect();

    parallel_colourings
}
```




