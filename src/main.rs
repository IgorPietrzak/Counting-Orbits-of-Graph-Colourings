use opp::{graph::Graph, opp::OPP};
use std::collections::HashSet;

// SAUCY PAPER: http://alcom.ee.ntu.edu.tw/system/privatezone/meetingfile/201210041839101.pdf

fn main() {
    let mut set = HashSet::new();
    for i in 0..3 {
        set.insert(i);
    }
    let graph = Graph::new(vec![vec![1], vec![0, 2], vec![1]]);
    let pi = graph.get_opp();
    pi.print();
    let new = pi.individualise(0, 2);
    println!("------------------------------------------------");
    new.print();
    println!("{}", new.is_isomorphic());
}
