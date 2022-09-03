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
