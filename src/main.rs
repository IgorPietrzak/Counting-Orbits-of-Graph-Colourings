use opp::graph::Graph;
use std::collections::HashSet;

// SAUCY PAPER: http://alcom.ee.ntu.edu.tw/system/privatezone/meetingfile/201210041839101.pdf

fn main() {
    let mut set = HashSet::new();
    set.insert(0);
    set.insert(1);
    set.insert(2);

    let graph = Graph::new(vec![vec![1, 2], vec![0, 2], vec![1, 0]]);
    println!("{:?}", graph);
}
