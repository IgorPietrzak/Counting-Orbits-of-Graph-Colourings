use crate::graph::Graph;

#[allow(non_snake_case)]
pub fn refine(graph: Graph) -> Vec<Vec<usize>> {
    // zeta = pi
    let mut pi = vec![vec![0, 1, 3], vec![2, 4]];

    let mut a = vec![vec![0, 1, 3], vec![2, 4]];
    let mut zeta = vec![vec![0, 1, 3], vec![2, 4]];
    let mut M = zeta.len();
    let mut m: usize = 0;

    while !(is_discrete(&zeta) || m >= M) {
        let W = a[m].clone();
        m = m + 1;

        // define (X1, ..., Xs)
        for k in 0..zeta.len() {
            // partition by degree of vertices w.r.t current W
            let x_tuple: Vec<Vec<usize>> = graph.generate_partition_of_Vk(&W, &zeta[k]);
            println!("W: {:?}, Vk: {:?}, xtup: {:?}", W, zeta[k], x_tuple);

            let s = x_tuple.len();
            if s == 1 {
                continue;
            }
            let Xt = get_Xt(&x_tuple);
            // set Wj = Xt
            for i in 0..pi.len() {
                for j in 0..a.len() {
                    if a[j] == zeta[i] {
                        a[j] = zeta[Xt].clone();
                    }
                }
            }
            // set  W_M+i
            for i in 0..Xt {
                a.push(x_tuple[i].clone());
            }
            // set W_M+i-1
            for i in Xt + 1..s {
                a.push(x_tuple[i].clone());
            }

            M = M + s - 1;

            // update zeta

            zeta.splice(k..=k, x_tuple);
        }
    }

    return zeta;
}
fn is_discrete(zeta: &Vec<Vec<usize>>) -> bool {
    for i in zeta {
        if i.len() > 1 {
            return false;
        }
    }
    true
}

// get index of Xt because that is closer to what algoritm does - get the "t".

#[allow(non_snake_case)]
pub fn get_Xt(x_tuple: &Vec<Vec<usize>>) -> usize {
    let mut max_length = 0;
    let mut Xt = 0;
    for i in 0..x_tuple.len() {
        if x_tuple[i].len() > max_length {
            Xt = i;
            max_length = x_tuple[i].len();
        }
    }
    Xt
}

#[cfg(test)]
mod test {
    use super::get_Xt;
    use super::refine;
    use crate::graph::Graph;
    #[test]
    fn test_refinement_algorithm() {
        let G = Graph::new(vec![
            vec![1, 2],
            vec![0, 2],
            vec![0, 1, 3],
            vec![2, 4],
            vec![3],
        ]);
        let W = vec![0, 1, 3];
        let res = G.generate_partition_of_Vk(&W.clone(), &W);
        let zeta = refine(G);
        println!("refined: {:?}", zeta);
    }
}
