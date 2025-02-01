use opp::{graph::Graph, opp::OPP};
use std::collections::HashSet;

// SAUCY PAPER: http://alcom.ee.ntu.edu.tw/system/privatezone/meetingfile/201210041839101.pdf

fn main() {
    let mut set = HashSet::new();
    for i in 0..3 {
        set.insert(i);
    }
    // first saucy example graph
    let graph = Graph::new(vec![vec![2], vec![2], vec![0, 1]]);
    let pi = graph.get_opp();
    pi.print();
    let new = pi.individualise(0, 1);
    println!("------------------------------------------------");
    new.print();
    println!("{}", new.is_discrete());
}
