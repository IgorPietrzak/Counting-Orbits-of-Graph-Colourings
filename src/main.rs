use opp::{graph::Graph, opp::OPP};
use std::collections::HashSet;

// SAUCY PAPER: http://alcom.ee.ntu.edu.tw/system/privatezone/meetingfile/201210041839101.pdf

fn main() {
    let mut set = HashSet::new();
    for i in 0..3 {
        set.insert(i);
    }
    let top = vec![vec![1], vec![0, 2]];
    let bottom = top.clone();
    let graph = Graph::new(vec![vec![1], vec![0, 2], vec![1]]);
    let pi = OPP { set, top, bottom };
    let mut algorithm_state = opp::algorithm::Algorithm::init(graph, pi);
    algorithm_state.print_current_state();
    algorithm_state.run();
    algorithm_state.print_current_state();
}
