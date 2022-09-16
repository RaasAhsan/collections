use std::{cmp::Ordering, fmt::Debug, ptr::NonNull};

/// An AVL tree is a self-balancing binary search tree.
/// Invariant: for any node N, the heights of both children of N may differ by no more than 1.
#[derive(Debug)]
pub enum AVLTree<K, V> {
    Node(Node<K, V>),
    Nil,
}

impl<K, V> AVLTree<K, V> {
    pub fn new() -> Self {
        Self::Nil
    }

    fn is_nil(&self) -> bool {
        match self {
            AVLTree::Node(_) => false,
            AVLTree::Nil => true,
        }
    }

    fn take_value(&mut self) -> Option<V> {
        match self {
            AVLTree::Node(node) => Some(node.entry.value.take().unwrap()),
            AVLTree::Nil => None,
        }
    }

    fn node_mut(&mut self) -> Option<&mut Node<K, V>> {
        match self {
            AVLTree::Node(node) => Some(node),
            AVLTree::Nil => None,
        }
    }

    pub fn balance_factor(&self) -> isize {
        match self {
            AVLTree::Node(node) => node.balance(),
            AVLTree::Nil => 0,
        }
    }

    pub fn height(&self) -> usize {
        match self {
            AVLTree::Node(node) => node.height_m,
            AVLTree::Nil => 0,
        }
    }
}

impl<K, V> AVLTree<K, V>
where
    K: Ord,
{
    pub fn get(&self, k: &K) -> Option<&V> {
        match self {
            AVLTree::Node(node) => unsafe {
                match k.cmp(&node.entry.key) {
                    Ordering::Equal => Some(node.entry.value.as_ref().unwrap()),
                    Ordering::Less => node.left.as_ref().get(k),
                    Ordering::Greater => node.right.as_ref().get(k),
                }
            },
            AVLTree::Nil => None,
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        unsafe {
            match self {
                AVLTree::Node(node) => {
                    match k.cmp(&node.entry.key) {
                        Ordering::Less => node.left.as_mut().insert(k, v),
                        Ordering::Greater => node.right.as_mut().insert(k, v),
                        Ordering::Equal => {}
                    }
                    node.update_height();

                    self.rebalance();
                }
                AVLTree::Nil => {
                    let node = Node {
                        entry: Entry::new(k, v),
                        left: NonNull::new_unchecked(Box::into_raw(Box::new(
                            AVLTree::<K, V>::new(),
                        ))),
                        right: NonNull::new_unchecked(Box::into_raw(Box::new(
                            AVLTree::<K, V>::new(),
                        ))),
                        height_m: 1,
                    };
                    *self = AVLTree::Node(node);
                }
            }
        }
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        match self {
            AVLTree::Node(node) => unsafe {
                match k.cmp(&node.entry.key) {
                    Ordering::Less => node.left.as_mut().remove(k),
                    Ordering::Greater => node.right.as_mut().remove(k),
                    Ordering::Equal => {
                        if let Some(replace) = node.right.as_mut().leftmost_mut() {
                            let replace_right = replace.node_mut().unwrap().right.as_mut();
                            std::mem::swap(replace, replace_right);
                            std::mem::swap(
                                &mut node.entry,
                                &mut replace_right.node_mut().unwrap().entry,
                            );
                            Some(replace_right.take_value().unwrap())
                        } else {
                            let mut replace = std::mem::take(node.left.as_mut());
                            std::mem::swap(self, &mut replace);
                            Some(replace.take_value().unwrap())
                        }
                    }
                }
            },
            AVLTree::Nil => None,
        }
    }

    fn leftmost_mut(&mut self) -> Option<&mut AVLTree<K, V>> {
        match self {
            AVLTree::Node(node) => unsafe {
                if node.left.as_ref().is_nil() {
                    Some(self)
                } else {
                    node.left.as_mut().leftmost_mut()
                }
            },
            AVLTree::Nil => None,
        }
    }

    fn rebalance(&mut self) {
        match self {
            AVLTree::Node(node) => match node.balance() {
                -2 => unsafe {
                    let left_ref = node.left.as_mut();
                    if left_ref.balance_factor() <= 0 {
                        self.unsafe_rotate_right()
                    } else {
                        left_ref.unsafe_rotate_left();
                        self.unsafe_rotate_right();
                    }
                },
                2 => unsafe {
                    let right_ref = node.right.as_mut();
                    if right_ref.balance_factor() >= 0 {
                        self.unsafe_rotate_left()
                    } else {
                        right_ref.unsafe_rotate_right();
                        self.unsafe_rotate_left();
                    }
                },
                -1 | 0 | 1 => {}
                _ => panic!("illegal balance factor"),
            },
            AVLTree::Nil => {}
        }
    }

    unsafe fn unsafe_rotate_right(&mut self) {
        let child = self.node_mut().unwrap().left.as_mut();
        let grandchild = child.node_mut().unwrap().right.as_mut();
        rotate(self, child, grandchild);
    }

    unsafe fn unsafe_rotate_left(&mut self) {
        let child = self.node_mut().unwrap().right.as_mut();
        let grandchild = child.node_mut().unwrap().left.as_mut();
        rotate(self, child, grandchild);
    }
}

/// Performs a left or right rotation.
/// Given a parent, child, and grandchild, perform a rotation
/// such that the parent and child swap positions and exchange the grandchild.
fn rotate<K, V>(
    parent: &mut AVLTree<K, V>,
    child: &mut AVLTree<K, V>,
    grandchild: &mut AVLTree<K, V>,
) {
    let mut temp = std::mem::take(grandchild);
    std::mem::swap(&mut temp, child); // temp has child now, grandchild has child now
    std::mem::swap(&mut temp, parent); // parent is child now, temp has old parent
    std::mem::swap(&mut temp, grandchild); // move old parent into new parent child
    grandchild.node_mut().unwrap().update_height();
    parent.node_mut().unwrap().update_height();
}

impl<K, V> Default for AVLTree<K, V> {
    fn default() -> Self {
        AVLTree::Nil
    }
}

impl<K, V> Drop for AVLTree<K, V> {
    fn drop(&mut self) {
        match self {
            AVLTree::Node(node) => unsafe {
                Box::from_raw(node.left.as_ptr());
                Box::from_raw(node.right.as_ptr());
            },
            AVLTree::Nil => {}
        }
    }
}

#[derive(Debug)]
pub struct Node<K, V> {
    entry: Entry<K, V>,
    left: NonNull<AVLTree<K, V>>,
    right: NonNull<AVLTree<K, V>>,
    height_m: usize,
}

#[derive(Debug)]
pub struct Entry<K, V> {
    key: K,
    value: Option<V>
}

impl<K, V> Entry<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Entry { key, value: Some(value) }
    }
}

impl<K, V> Node<K, V> {
    fn update_height(&mut self) {
        unsafe {
            self.height_m =
                1 + std::cmp::max(self.left.as_ref().height(), self.right.as_ref().height())
        }
    }

    fn balance(&self) -> isize {
        unsafe { (self.right.as_ref().height() as isize) - (self.left.as_ref().height() as isize) }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;
    use std::collections::HashSet;

    use crate::avl_tree::AVLTree;

    impl<K, V> AVLTree<K, V> {
        fn height_internal(&self) -> usize {
            unsafe {
                match self {
                    AVLTree::Node(node) => {
                        1 + std::cmp::max(
                            node.left.as_ref().height_internal(),
                            node.right.as_ref().height_internal(),
                        )
                    }
                    AVLTree::Nil => 0,
                }
            }
        }

        fn balanced_internal(&self) -> bool {
            unsafe {
                match self {
                    AVLTree::Node(node) => {
                        let left = node.left.as_ref();
                        let right = node.right.as_ref();
                        left.balanced_internal()
                            && right.balanced_internal()
                            && ((left.height_internal() as isize)
                                - (right.height_internal() as isize))
                                .abs()
                                <= 1
                    }
                    AVLTree::Nil => true,
                }
            }
        }
    }

    fn test_insertion_balance(input: Vec<i32>) {
        let mut tree = AVLTree::<i32, i32>::new();
        for i in input.iter() {
            tree.insert(*i, *i);
        }
        assert!(tree.balanced_internal());
        for i in input.iter() {
            assert_eq!(tree.get(i), Some(i));
        }
    }

    #[test]
    fn insert_and_get() {
        let mut tree = AVLTree::new();
        tree.insert(10, 10);
        assert_eq!(tree.get(&10), Some(&10));
        assert_eq!(tree.get(&9), None);
    }

    #[test]
    fn rotat_right() {
        test_insertion_balance(vec![10, 5, 0]);
        test_insertion_balance(vec![15, 10, 20, 5, 0]);
        test_insertion_balance(vec![15, 10, 20, 5, 12, 0]);
    }

    #[test]
    fn rotat_left() {
        test_insertion_balance(vec![0, 5, 10]);
        test_insertion_balance(vec![15, 10, 20, 25, 30]);
        test_insertion_balance(vec![15, 10, 20, 18, 25, 30]);
        test_insertion_balance(vec![15, 10, 20, 18, 25, 22]);
    }

    #[test]
    fn rotat_right_left() {
        test_insertion_balance(vec![15, 10, 20, 18, 25, 19]);
    }

    #[test]
    fn rotat_left_right() {
        test_insertion_balance(vec![15, 10, 20, 5, 12, 14]);
    }

    #[test]
    fn remove_left() {
        let mut tree = AVLTree::new();
        tree.insert(5, 5);
        tree.insert(2, 2);
        assert_eq!(tree.remove(&5), Some(5));
        assert_eq!(tree.get(&5), None);
        assert_eq!(tree.get(&2), Some(&2));
    }

    #[test]
    fn remove_right() {
        let mut tree = AVLTree::new();
        tree.insert(5, 5);
        tree.insert(2, 2);
        tree.insert(7, 7);
        assert_eq!(tree.remove(&5), Some(5));
        assert_eq!(tree.get(&5), None);
        assert_eq!(tree.get(&2), Some(&2));
        assert_eq!(tree.get(&7), Some(&7));
    }

    #[test]
    fn remove_right_leftmost() {
        let mut tree = AVLTree::new();
        tree.insert(5, 5);
        tree.insert(2, 2);
        tree.insert(7, 7);
        tree.insert(6, 6);
        assert_eq!(tree.remove(&5), Some(5));
        assert_eq!(tree.get(&5), None);
        assert_eq!(tree.get(&6), Some(&6));
        assert_eq!(tree.get(&2), Some(&2));
        assert_eq!(tree.get(&7), Some(&7));
    }

    #[test]
    fn prop_tree_insertion() {
        fn p(input: HashSet<i32>) -> bool {
            let mut tree = AVLTree::new();
            for i in input.iter() {
                tree.insert(*i, *i);
            }
            input.iter().all(|i| tree.get(i).is_some())
        }
        quickcheck(p as fn(HashSet<i32>) -> bool)
    }

    #[test]
    fn prop_tree_balance() {
        fn p(input: HashSet<i32>) -> bool {
            let mut tree = AVLTree::new();
            for i in input.iter() {
                tree.insert(*i, *i);
            }
            tree.balanced_internal()
        }
        quickcheck(p as fn(HashSet<i32>) -> bool)
    }
}
