use std::collections::HashSet;

use crate::opp::OPP;

// use an adjacency matrix as it is more effcient than a hashmap.
#[derive(Debug)]
pub struct Graph {
    pub adj_mat: Vec<Vec<usize>>,
    pub vertices: Vec<usize>,
}
impl Graph {
    pub fn new(adj_mat: Vec<Vec<usize>>) -> Self {
        let vertices: Vec<usize> = (0..adj_mat.len()).collect();
        Graph { adj_mat, vertices }
    }

    // degree based pruning decides what OPP we begin with. O(n) - No need for this we get orbit partition from saucy.
    pub fn get_opp(&self) -> OPP {
        let mut set: HashSet<usize> = HashSet::new();
        for i in 0..self.adj_mat.len() {
            set.insert(i);
        }
        let compatibility_vec = self.sort_by_degree();
        let mut top: Vec<Vec<usize>> = Vec::new();
        let mut bottom: Vec<Vec<usize>> = Vec::new();
        for i in compatibility_vec {
            top.push(i.clone());
            bottom.push(i);
        }
        OPP::new(set, top, bottom)
    }
    // Multithread this??
    fn sort_by_degree(&self) -> Vec<Vec<usize>> {
        let mut seen: Vec<usize> = Vec::new();
        let mut compatibility_vec: Vec<Vec<usize>> = Vec::new();

        for i in 0..self.adj_mat.len() {
            if seen.len() == self.adj_mat.len() {
                break;
            }
            let mut compatible_nodes = Vec::new();
            compatible_nodes.push(i);
            seen.push(i);

            for j in 0..self.adj_mat.len() {
                if !seen.contains(&j) {
                    // .contains() is O(n)
                    match self.adj_mat[i].len() == self.adj_mat[j].len() {
                        true => {
                            seen.push(j);
                            compatible_nodes.push(j);
                        }
                        false => {}
                    }
                }
            }
            compatibility_vec.push(compatible_nodes);
        }

        compatibility_vec
    }
}

// #[cfg(test)]
// mod test {
//     use super::Graph;

//     #[test]
//     fn test_sorting_by_degree() {
//         let graph = Graph::new(vec![vec![1], vec![0, 2], vec![1]]);
//         println!("sorted {:?}", graph.sort_by_degree());
//     }
// }
