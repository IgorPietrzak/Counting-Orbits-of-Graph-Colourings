use opp::graph::Graph;

// SAUCY PAPER: http://alcom.ee.ntu.edu.tw/system/privatezone/meetingfile/201210041839101.pdf

fn main() {
    let graph = Graph::new(vec![vec![1, 2], vec![0, 2], vec![1, 0]]);
    let opp = graph.get_opp();
    opp.print();
}
