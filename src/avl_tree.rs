use std::cmp::Ordering;

#[derive(Debug)]
pub enum AVLTree<A> {
    Node {
        value: A,
        left: Box<AVLTree<A>>,
        right: Box<AVLTree<A>>,
    },
    Nil,
}

impl<A> AVLTree<A> {
    pub fn new() -> Self {
        AVLTree::Nil
    }
}

impl<A> Default for AVLTree<A> {
    fn default() -> Self {
        AVLTree::Nil
    }
}

impl<A> AVLTree<A>
where
    A: Ord,
{
    pub fn search(&self, a: A) -> Option<&AVLTree<A>> {
        match self {
            AVLTree::Node { value, left, right } => match a.cmp(value) {
                Ordering::Less => left.search(a),
                Ordering::Equal => Some(&self),
                Ordering::Greater => right.search(a),
            },
            AVLTree::Nil => None,
        }
    }

    pub fn insert(&mut self, a: A) -> bool {
        match self {
            AVLTree::Node { value, left, right } => match a.cmp(value) {
                Ordering::Less => left.insert(a),
                Ordering::Equal => true,
                Ordering::Greater => right.insert(a),
            },
            AVLTree::Nil => {
                *self = AVLTree::Node {
                    value: a,
                    left: Box::new(AVLTree::Nil),
                    right: Box::new(AVLTree::Nil),
                };
                false
            }
        }
    }

    pub fn is_node(&self) -> bool {
        match self {
            AVLTree::Node {
                value: _,
                left: _,
                right: _,
            } => true,
            AVLTree::Nil => false,
        }
    }

    pub fn remove(&mut self, a: A) -> bool {
        match self {
            AVLTree::Node { value, left, right } => match a.cmp(value) {
                Ordering::Less => left.remove(a),
                Ordering::Equal => {
                    match (left.is_node(), right.is_node()) {
                        (true, true) => right.swap_leftmost(value),
                        (true, false) => *self = std::mem::take(left),
                        (false, true) => *self = std::mem::take(right),
                        (false, false) => *self = AVLTree::Nil,
                    }
                    true
                }
                Ordering::Greater => right.remove(a),
            },
            AVLTree::Nil => false,
        }
    }

    fn swap_leftmost(&mut self, to: &mut A) {
        match self {
            AVLTree::Node { value, left, right } => {
                if !left.is_node() {
                    std::mem::swap(value, to);
                    *self = std::mem::take(right);
                } else {
                    left.swap_leftmost(to);
                }
            }
            AVLTree::Nil => {}
        }
    }

    pub fn height(&self) -> usize {
        match self {
            AVLTree::Node {
                value: _,
                left,
                right,
            } => std::cmp::max(left.height(), right.height()),
            AVLTree::Nil => 0,
        }
    }

    pub fn balance(&self) -> i16 {
        match self {
            AVLTree::Node {
                value: _,
                left,
                right,
            } => (right.height() as i16) - (left.height() as i16),
            AVLTree::Nil => 0,
        }
    }

    /// Returns an iterator that traverses the keys of the tree in ascending order
    pub fn iter<'a>(&self) -> Iter<'a, A> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Iter<'a, A> {
    tree: &'a AVLTree<A>,
}

impl<'a, A> Iterator for Iter<'a, A> {
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::AVLTree;

    #[test]
    fn tree_search() {
        let mut tree = AVLTree::new();
        tree.insert(3);
        assert!(tree.search(3).is_some());
    }
}
