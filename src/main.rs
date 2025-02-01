use ::opp::graph::Graph;
use std::collections::HashSet;

// SAUCY PAPER: http://alcom.ee.ntu.edu.tw/system/privatezone/meetingfile/201210041839101.pdf

fn main() {
    let mut set = HashSet::new();
    for i in 0..3 {
        set.insert(i);
    }
    // first saucy example graph
    let graph = Graph::new(vec![vec![2], vec![2], vec![0, 1]]);
    // 2nd big example from saucy
    let graph2 = Graph::new(vec![
        vec![1, 3],
        vec![0, 2],
        vec![1, 3],
        vec![0, 2],
        vec![6, 5],
        vec![4, 6],
        vec![4, 5],
    ]);
    let permutations = graph2.get_automorphisms();
    println!("Aut(G) = {:?}", permutations);
}
