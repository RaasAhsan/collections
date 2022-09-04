use std::cmp::Ordering;

#[derive(Debug)]
pub enum BSTree<A> {
    Node {
        value: A,
        left: Box<BSTree<A>>,
        right: Box<BSTree<A>>,
    },
    Nil,
}

impl<A> BSTree<A> {
    pub fn new() -> Self {
        BSTree::Nil
    }
}

impl<A> Default for BSTree<A> {
    fn default() -> Self {
        BSTree::Nil
    }
}

impl<A> BSTree<A>
where
    A: Ord,
{
    pub fn search(&self, a: A) -> Option<&BSTree<A>> {
        match self {
            BSTree::Node { value, left, right } => match a.cmp(value) {
                Ordering::Less => left.search(a),
                Ordering::Equal => Some(&self),
                Ordering::Greater => right.search(a),
            },
            BSTree::Nil => None,
        }
    }

    pub fn insert(&mut self, a: A) -> bool {
        match self {
            BSTree::Node { value, left, right } => match a.cmp(value) {
                Ordering::Less => left.insert(a),
                Ordering::Equal => true,
                Ordering::Greater => right.insert(a),
            },
            BSTree::Nil => {
                *self = BSTree::Node {
                    value: a,
                    left: Box::new(BSTree::Nil),
                    right: Box::new(BSTree::Nil),
                };
                false
            }
        }
    }

    pub fn is_node(&self) -> bool {
        match self {
            BSTree::Node {
                value: _,
                left: _,
                right: _,
            } => true,
            BSTree::Nil => false,
        }
    }

    pub fn remove(&mut self, a: A) -> bool {
        match self {
            BSTree::Node { value, left, right } => match a.cmp(value) {
                Ordering::Less => left.remove(a),
                Ordering::Equal => {
                    match (left.is_node(), right.is_node()) {
                        (true, true) => right.swap_leftmost(value),
                        (true, false) => *self = std::mem::take(left),
                        (false, true) => *self = std::mem::take(right),
                        (false, false) => *self = BSTree::Nil,
                    }
                    true
                }
                Ordering::Greater => right.remove(a),
            },
            BSTree::Nil => false,
        }
    }

    fn swap_leftmost(&mut self, to: &mut A) {
        match self {
            BSTree::Node { value, left, right } => {
                if !left.is_node() {
                    std::mem::swap(value, to);
                    *self = std::mem::take(right);
                } else {
                    left.swap_leftmost(to);
                }
            }
            BSTree::Nil => {}
        }
    }

    pub fn height(&self) -> usize {
        match self {
            BSTree::Node {
                value: _,
                left,
                right,
            } => std::cmp::max(left.height(), right.height()),
            BSTree::Nil => 0,
        }
    }

    pub fn balance(&self) -> i16 {
        match self {
            BSTree::Node {
                value: _,
                left,
                right,
            } => (right.height() as i16) - (left.height() as i16),
            BSTree::Nil => 0,
        }
    }

    /// Returns an iterator that traverses the keys of the tree in ascending order
    pub fn iter<'a>(&self) -> Iter<'a, A> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Iter<'a, A> {
    tree: &'a BSTree<A>,
}

impl<'a, A> Iterator for Iter<'a, A> {
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::BSTree;

    #[test]
    fn tree_search() {
        let mut tree = BSTree::new();
        tree.insert(3);
        assert!(tree.search(3).is_some());
    }
}
