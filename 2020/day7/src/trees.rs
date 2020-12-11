use std::collections::HashSet;

#[derive(Debug)]
pub struct Node<T>
where
    T: PartialEq,
{
    pub idx: usize,
    pub val: T,
    pub parent: HashSet<usize>,
    pub children: HashSet<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: HashSet::new(),
            children: HashSet::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ArenaTree<T>
where
    T: PartialEq,
{
    pub arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    pub fn link_parent_child(&mut self, parent: usize, child: usize) {
        self.arena[parent].children.insert(child);
        self.arena[child].parent.insert(parent);
    }

    pub fn node(&mut self, val: T) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val));
        idx
    }

    #[cfg(test)]
    pub fn size(&self) -> usize {
        self.arena.len()
    }
}
