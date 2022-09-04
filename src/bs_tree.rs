use std::cmp::Ordering;

/// An unbalanced binary search tree.
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
                        (true, true) => right.swap_leftmost(value), // Swap the current node with its immediate successor
                        (true, false) => *self = std::mem::take(left), // Promote the left subtree
                        (false, true) => *self = std::mem::take(right), // Promote the right subtree
                        (false, false) => *self = BSTree::Nil,      // Clear out the current node
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
            } => std::cmp::max(left.height(), right.height()) + 1,
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

    pub fn value(&self) -> Option<&A> {
        match self {
            BSTree::Node {
                value,
                left: _,
                right: _,
            } => Some(value),
            BSTree::Nil => None,
        }
    }

    /// Returns an iterator that traverses the keys of the tree in ascending order.
    /// This corresponds to an in-order traveral of the tree.
    pub fn iter<'a>(&'a self) -> Iter<'a, A> {
        Iter {
            state: IterState::Left,
            tree: self,
            parent: None,
        }
    }
}

#[derive(Debug)]
pub struct Iter<'a, A> {
    state: IterState,
    tree: &'a BSTree<A>,
    parent: Option<Box<Iter<'a, A>>>,
}

impl<'a, A> Iter<'a, A>
where
    A: Ord,
{
    fn continue_to_parent(&mut self) -> Option<&'a A> {
        match self.parent.take() {
            Some(mut p) => {
                std::mem::swap(self, &mut p);
                self.next()
            }
            None => None,
        }
    }
}

#[derive(Debug)]
enum IterState {
    Left,
    Node,
    Right,
}

impl<'a, A> Iterator for Iter<'a, A>
where
    A: Ord,
{
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tree {
            BSTree::Node { value, left, right } => match self.state {
                IterState::Left => {
                    self.state = IterState::Node;
                    let mut new_parent = left.iter();
                    std::mem::swap(self, &mut new_parent);
                    self.parent = Some(Box::new(new_parent));
                    self.next()
                }
                IterState::Node => {
                    self.state = IterState::Right;
                    let mut new_parent = right.iter();
                    std::mem::swap(self, &mut new_parent);
                    self.parent = Some(Box::new(new_parent));
                    Some(value)
                }
                IterState::Right => self.continue_to_parent(),
            },
            BSTree::Nil => self.continue_to_parent(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::BSTree;

    #[test]
    fn tree_search() {
        let mut tree = BSTree::new();
        tree.insert(3);
        tree.insert(4);
        assert!(tree.search(3).is_some());
        assert!(tree.search(4).is_some());
    }

    #[test]
    fn tree_removal() {
        let mut tree = BSTree::new();
        tree.insert(3);
        tree.insert(4);
        assert!(tree.search(3).is_some());
        assert_eq!(tree.remove(4), true);
    }

    #[test]
    fn tree_height() {
        let mut tree = BSTree::new();
        tree.insert(5);
        tree.insert(4);
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);
        tree.insert(0);
        assert_eq!(tree.height(), 6);
    }

    #[test]
    fn tree_iteration() {
        let mut tree = BSTree::new();
        tree.insert(4);
        tree.insert(3);
        tree.insert(5);
        tree.insert(0);
        tree.insert(2);
        tree.insert(1);
        let mut iter = tree.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);
    }
}
