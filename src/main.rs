use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(0);
    set.insert(1);
    set.insert(2);
    let top: vec![]
}

#[derive(Debug)]
struct OPP {
    pub set: HashSet<usize>,
    pub top: Vec<HashSet<usize>>,
    pub bottom: Vec<HashSet<usize>>,
}

impl OPP {
    pub fn print(&self) {
        println!("{{");
        println!("{:?}", self.top);
        println!("}}");
    }

    pub fn new(set: HashSet<usize>,top: Vec<usize>, bottom: Vec<usize>) -> Self{
        

        
    }
}
