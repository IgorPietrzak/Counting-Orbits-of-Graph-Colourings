use std::collections::{HashMap, HashSet};
mod opp;

// SAUCY PAPER: http://alcom.ee.ntu.edu.tw/system/privatezone/meetingfile/201210041839101.pdf

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

// IDEAS:
/*
- Make a graph struct to and in the constructor compute the degree of each vertex,
  this should be sufficient for the partition refinement.
- How do we verify that an automorphism is valid? - Go from the definition?
- Before creating a new layer of branches, make sure the OPP is isomorphic.
*/
