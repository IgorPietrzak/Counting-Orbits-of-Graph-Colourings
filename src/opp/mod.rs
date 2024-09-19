use std::collections::HashSet;
#[derive(Debug)]
pub struct OPP {
    set: HashSet<usize>,
    pub top: Vec<Vec<usize>>,
    pub bottom: Vec<Vec<usize>>,
}

// isomorphic hopefully - need to have a check for non-isomorphic ones
impl OPP {
    pub fn print(&self) {
        println!("{:?}", self.top);
        println!("{:?}", self.bottom);
    }

    pub fn new(set: HashSet<usize>, top: Vec<Vec<usize>>, bottom: Vec<Vec<usize>>) -> Self {
        Self { set, top, bottom }
    }

    pub fn create_branch(
        &self,
        target_cell: &usize,
        target_vertex: &usize,
        maps_to: &usize,
    ) -> Self {
        let top_cell = self.top[*target_cell].clone();
        let bottom_cell = self.bottom[*target_cell].clone();
        let vertex = top_cell[*target_vertex];
        let mapped = bottom_cell[*maps_to];
        // cells before target stay the same, on top and bottom:
        let mut new_top_op: Vec<Vec<usize>> = Vec::new();
        let mut new_bottom_op: Vec<Vec<usize>> = Vec::new();
        let mut i = 0;
        while &i < target_cell {
            new_top_op.push(self.top[i].clone());
            new_bottom_op.push(self.bottom[i].clone());
            i += 1;
        }

        // The next cells are target cells with target vertices removed:
        let without_target_top = remove_element(top_cell, target_vertex);
        let without_target_bottom = remove_element(bottom_cell, maps_to);
        new_top_op.push(without_target_top);
        new_bottom_op.push(without_target_bottom);

        // Insert the singleton target vertex cells:
        new_top_op.push(vec![vertex]);
        new_bottom_op.push(vec![mapped]);

        // insert the cells after the target cell:
        let mut i = *target_cell + 1;
        while i < self.top.len() {
            new_top_op.push(self.top[i].clone());
            new_bottom_op.push(self.bottom[i].clone());
            i += 1;
        }
        Self {
            set: self.set.clone(),
            top: new_top_op,
            bottom: new_bottom_op,
        }
    }
}

// HELPERS:
fn remove_element(vec: Vec<usize>, index: &usize) -> Vec<usize> {
    vec.into_iter()
        .enumerate()
        .filter(|(i, _)| *i != *index)
        .map(|(_, elem)| elem)
        .collect()
}

#[cfg(test)]
mod test {
    use super::remove_element;
    #[test]
    fn test_remove() {
        println!("{:?}", remove_element(vec![0, 1, 2], &0));
    }
}
