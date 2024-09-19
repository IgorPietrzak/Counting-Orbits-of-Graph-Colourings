use std::collections::{HashMap, HashSet};
mod opp;

fn main() {
    let mut set = HashSet::new();
    set.insert(0);
    set.insert(1);
    set.insert(2);
    let opp = opp::OPP::new(set, vec![vec![0, 1, 2]], vec![vec![0, 1, 2]]);
    opp.print();
    let branch = opp.create_branch(&0, &1, &0);
    branch.print();
}

pub struct Graph {
    pub vertices: HashSet<usize>,
    pub hashmap_rep: HashMap<usize, Vec<usize>>,
}
