use ndarray::Array2;
use std::cmp::Ordering;

/*

We can check if automorphism is valid in O(n^2) time.
If P and A are permutation and adjacency matrices respectively,
A = PAP^-1 guarantees that P is an automorphism.


*/

pub fn check_automorphism(adj_list: &Vec<Vec<usize>>, perm: &Vec<usize>) -> bool {
    let adj_matrix = get_adj_mat(adj_list);
    let perm_matrix = get_perm_mat(perm);

    let perm_matrix_inv = perm_matrix.t().to_owned();

    let pap_inv = perm_matrix.dot(&adj_matrix).dot(&perm_matrix_inv);

    pap_inv == adj_matrix
}

fn get_adj_mat(adj_list: &Vec<Vec<usize>>) -> Array2<i32> {
    let n = adj_list.len();
    let mut adj_matrix = Array2::<i32>::zeros((n, n));
    for (i, neighbors) in adj_list.iter().enumerate() {
        for &j in neighbors {
            adj_matrix[[i, j]] = 1;
        }
    }

    adj_matrix
}

fn get_perm_mat(perm: &Vec<usize>) -> Array2<i32> {
    let n = perm.len();
    let mut perm_matrix = Array2::<i32>::zeros((n, n));
    for i in 0..perm.len() {
        perm_matrix[[i, perm[i]]] = 1;
    }
    perm_matrix
}

#[cfg(test)]
mod test {
    use super::check_automorphism;
    #[test]
    fn automorphism() {
        let adj_list = vec![vec![1], vec![0, 2], vec![1]];
        let adj_list_big = vec![
            vec![1, 3],
            vec![0, 2],
            vec![1, 3],
            vec![0, 2],
            vec![6, 5],
            vec![4, 6],
            vec![4, 5],
        ];
        // clearly, 1 maps to 1 in any automorphism
        let perm1 = vec![2, 1, 0];
        let perm2 = vec![0, 1, 2];
        let perm3 = vec![1, 0, 2];
        let perm_big_valid = vec![0, 1, 2, 3, 6, 5, 4];
        let perm_big_invalid = vec![4, 1, 2, 3, 0, 5, 6];
        assert_eq!(check_automorphism(&adj_list, &perm1), true);
        assert_eq!(check_automorphism(&adj_list, &perm2), true);
        assert_eq!(check_automorphism(&adj_list, &perm3), false);
        assert_eq!(check_automorphism(&adj_list_big, &perm_big_valid), true);
        assert_eq!(check_automorphism(&adj_list_big, &perm_big_invalid), false);
    }
}
