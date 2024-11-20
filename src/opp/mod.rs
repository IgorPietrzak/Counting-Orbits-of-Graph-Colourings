use std::collections::HashSet;

#[derive(Debug)]
pub struct OPP {
    pub set: HashSet<usize>,
    pub top: Vec<Vec<usize>>,
    pub bottom: Vec<Vec<usize>>,
}

impl OPP {
    pub fn print(&self) {
        println!("{:?}", self.top);
        println!("{:?}", self.bottom);
    }

    pub fn new(set: HashSet<usize>, top: Vec<Vec<usize>>, bottom: Vec<Vec<usize>>) -> Self {
        Self { set, top, bottom }
    }

    pub fn is_isomorphic(&self) -> bool {
        match self.top.len() == self.bottom.len() {
            true => {
                for (i, j) in self.top.iter().zip(self.bottom.iter()) {
                    if i.len() != j.len() {
                        return false;
                    }
                }
                true
            }
            false => false,
        }
    }

    pub fn is_matching(&self) -> bool {
        for (cell_t, cell_b) in std::iter::zip(self.top.clone(), self.bottom.clone()) {
            if cell_t.len() > 1 {
                if cell_t != cell_b {
                    return false;
                }
            }
        }
        true
    }
}

// #[cfg(test)]
// mod test {
//     use super::remove_element;
//     #[test]
//     fn test_remove() {
//         println!("{:?}", remove_element(vec![0, 1, 2], &0));
//     }
//     #[test]
//     fn test_range() {
//         println!("RANGE: {:?}", 0..3);
//         for i in 0..3 {
//             println!("{:?}", i);
//         }
//     }
// }
