use crate::opp::OPP;
use rayon::prelude::*; //multithreading lib

fn infection_set_orbits(opp: OPP) -> Vec<Vec<Vec<usize>>> {
    let cells: Vec<Vec<usize>> = opp.top;
    cells
        .par_iter() // run iterations in parallel
        .map(|cell| {
            let mut cell_colourings: Vec<Vec<usize>> = Vec::with_capacity(cell.len()); // preallocate sizeof(cell.len()) bytes when initialsing vec. - Better performance.
            for i in 0..cell.len() {
                let mut colouring = vec![0; cell.len()];
                colouring[i] = 1;
                cell_colourings.push(colouring);
            }
            cell_colourings
        })
        .collect()
}

pub fn infection_set(current_reps: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut infection_set = Vec::new();
    for colouring in current_reps {
        infection_set.extend(build_infection(colouring));
    }
    infection_set
}

fn build_infection(colouring: &Vec<usize>) -> Vec<Vec<usize>> {
    let parallel_colourings: Vec<Vec<usize>> = (0..colouring.len())
        .into_par_iter() // creates parallel iterator.
        .filter_map(|i| {
            if colouring[i] == 0 {
                let mut new_colouring = colouring.clone(); // We only clone when we need to flip 0 to 1.
                new_colouring[i] = 1;
                Some(new_colouring)
            } else {
                None // Do nothing when encounter a 1.
            }
        })
        .collect();

    parallel_colourings
}

// #[cfg(test)]
// mod test {
//     use super::infection_set;
//     #[test]
//     fn test_infection() {
//         let reps = vec![vec![0, 0, 0, 0], vec![1, 0, 0, 0]];
//         println!("INFECTION SET: {:?}", infection_set(&reps));
//     }
// }
