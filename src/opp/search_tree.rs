// The DFS permutation search tree is implemented here:
use super::OPP;
use crate::algorithm::automorphism::check_automorphism;
use crate::graph::Graph;
use std::collections::HashMap;

// Need to create a new struct which captures permuations
pub fn get_permuations(opp: &OPP, graph: &Graph) -> Vec<Vec<usize>> {
    // get target vertex
    let adj_mat = graph.adj_mat.clone();
    let mut permuatations = Vec::new();
    if opp.is_discrete() {
        let permutation = read_permuation(&opp);
        if check_automorphism(&adj_mat, &permutation) {
            permuatations.push(permutation);
            return permuatations;
        }
    }
    let root_target_cell = opp.get_target_cell();

    let mut stack: Vec<(OPP, usize)> = Vec::new();
    stack.push((opp.clone(), root_target_cell));

    while stack.len() > 0 {
        let curr_node = stack.pop().unwrap(); // LIFO
                                              // only add the permutation if it is an automorphism.
        if curr_node.0.is_discrete() {
            let permutation = read_permuation(&curr_node.0);
            if check_automorphism(&adj_mat, &permutation) {
                permuatations.push(permutation);
                continue;
            } else {
                continue;
            }
        }
        // USE MCKAY REFINEMENT HERE TO CHECK IF IT REFINES ISOMORPHICALLY
        let children_nodes = get_children(curr_node.0, curr_node.1);
        stack.extend(children_nodes);
    }

    permuatations
}

// given discrete OPP read the permuation
fn read_permuation(opp: &OPP) -> Vec<usize> {
    let mut permutation: Vec<usize> = Vec::new();
    let mut perm_map: HashMap<usize, usize> = HashMap::new();
    let top = opp.top.clone();
    let bottom = opp.bottom.clone();
    for i in 0..top.len() {
        perm_map.insert(top[i][0], bottom[i][0]);
    }
    for i in 0..perm_map.len() {
        // insert the value of perm_map[i] this will put the vertices in order
        permutation.push(perm_map[&i]);
    }
    permutation
}

// Take in non-discrete OPP
fn get_children(opp: OPP, target_index: usize) -> Vec<(OPP, usize)> {
    // the seeds are whatever we can map target to in bottom partition. i.e. the whole bottom[i]
    let child_seeds = opp.bottom[target_index].clone();
    let mut children: Vec<(OPP, usize)> = Vec::new();
    for seed in child_seeds {
        let child = opp.individualise(opp.top[target_index][0], seed).clone();
        let target_cell = child.get_target_cell();
        children.push((child, target_cell));
    }
    children
}
