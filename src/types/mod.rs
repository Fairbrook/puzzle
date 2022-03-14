pub mod ordered;
pub mod puzzle;
use crate::types::ordered::List;

pub trait Node: Clone {
    fn equ(&self, a: &Self) -> bool;
    fn expand<'a>(&'a self) -> Vec<Box<Self>>;
    fn to_string(&self) -> String;
    fn get_data(&self) -> i64;
}

pub struct Tree<T: Node> {
    pub root: T,
}

impl<T: Node> Tree<T> {
    pub fn breadth_first_search(&self, goal: &T) -> Result<T, ()> {
        let mut parents = vec![Box::new(self.root.clone())];
        let mut expanded = List::new();
        expanded.insert(self.root.get_data());
        if self.root.equ(goal){
            return Ok(self.root.clone())
        }
        loop {
            let mut next_parents = vec![];
            // {
            //     println!(
            //         "{}",
            //         parents
            //             .iter()
            //             .map(|parent| parent.get_data().to_string())
            //             .collect::<Vec<String>>()
            //             .join("\n")
            //     );
            // }
            for parent in &parents {
                let children = parent.expand();
                for child in children {
                    if child.equ(goal) {
                        return Ok(*child);
                    }
                    if !expanded.includes(child.get_data()) {
                        expanded.insert(child.get_data());
                        next_parents.push(child);
                    }
                }
            }
            parents = next_parents;
            if parents.len() == 0 {
                return Err(());
            }
        }
    }
}
