use std::collections::HashSet;
mod search_tree;

#[derive(Debug, Clone)]
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

    pub fn is_discrete(&self) -> bool {
        if !self.is_isomorphic() {
            return false;
        }
        for i in self.top.iter() {
            if i.len() != 1 {
                return false;
            }
        }
        true
    }

    pub fn individualise(&self, target_vertex: usize, maps_to: usize) -> Self {
        // find the target cell that target vertex is in.
        let mut target_cell = 0;
        for i in 0..self.top.len() {
            if self.top[i].contains(&target_vertex) {
                target_cell = i
            }
        }

        // remove target vertex from target cell
        let new_cell_top = remove_element(self.top[target_cell].clone(), &target_vertex);
        let singleton_top = vec![target_vertex];

        // same for corresponding bottom cell
        let new_cell_bot = remove_element(self.bottom[target_cell].clone(), &maps_to);
        let singleton_bot = vec![maps_to];

        // make the new opp

        let mut new_top = self.top.clone();
        let mut new_bottom = self.bottom.clone();

        // remove old target cell:
        new_top.remove(target_cell);
        new_bottom.remove(target_cell);

        // build new top and bottom

        new_top.insert(target_cell, new_cell_top);
        new_bottom.insert(target_cell, new_cell_bot);

        // Need to handle case where target_cell + 1 could be out of bounds.
        if target_cell == new_top.len() - 1 {
            new_top.push(singleton_top);
            new_bottom.push(singleton_bot);
        } else {
            new_top.insert(target_cell + 1, singleton_top);
            new_bottom.insert(target_cell + 1, singleton_bot);
        }
        Self {
            set: self.set.clone(),
            top: new_top,
            bottom: new_bottom,
        }
    }

    pub fn get_target_cell(&self) -> usize {
        let mut target_index = 0;
        for i in 0..self.top.len() {
            if self.top[i].len() > 1 {
                target_index = i;
                break;
            }
        }

        // return first valid target vertex - this is a heuristic maybe there's randomness stuff that makes this better.
        target_index
    }
}

fn remove_element(vec: Vec<usize>, target: &usize) -> Vec<usize> {
    vec.into_iter()
        .enumerate()
        .filter(|(_, i)| *i != *target)
        .map(|(_, elem)| elem)
        .collect()
}

#[cfg(test)]
mod test {
    use super::remove_element;
    #[test]
    fn test_remove() {
        println!(
            "REMOVE ELEMENT RES: {:?}",
            remove_element(vec![0, 1, 2], &0)
        );
    }
    #[test]
    fn test_range() {
        println!("RANGE: {:?}", 0..3);
        for i in 0..3 {
            println!("{:?}", i);
        }
    }
}
