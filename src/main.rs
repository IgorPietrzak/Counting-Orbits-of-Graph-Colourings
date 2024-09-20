use std::collections::{HashMap, HashSet};
mod opp;

fn main() {
    let mut set = HashSet::new();
    set.insert(0);
    set.insert(1);
    set.insert(2);
    let opp = opp::OPP::new(set, vec![vec![0, 1, 2]], vec![vec![0, 1, 2]]);
    println!("{:?}", opp.is_isomorphic());
    let branches = opp.create_branches(&0, &1);
    for i in branches {
        i.print();
        println!("\n");
    }
}

pub struct Graph {
    pub vertices: HashSet<usize>,
    pub hashmap_rep: HashMap<usize, Vec<usize>>,
}
