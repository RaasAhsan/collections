use std::{cmp::Ordering, fmt::Debug, ptr::NonNull};

#[derive(Debug)]
pub enum AVLTree<K, V> {
    Node(Node<K, V>),
    Nil,
}

impl<K, V> AVLTree<K, V> {
    pub fn new() -> Self {
        Self::Nil
    }

    fn node(&self) -> Option<&Node<K, V>> {
        match self {
            AVLTree::Node(node) => Some(node),
            AVLTree::Nil => None,
        }
    }

    pub fn height(&self) -> usize {
        match self {
            AVLTree::Node(node) => node.height_m,
            AVLTree::Nil => 0,
        }
    }

    fn reset_height(&mut self) {
        unsafe {
            match self {
                AVLTree::Node(node) => {
                    node.height_m =
                        1 + std::cmp::max(node.left.as_ref().height(), node.right.as_ref().height())
                }
                AVLTree::Nil => {}
            }
        }
    }

    pub fn balance(&self) -> isize {
        unsafe {
            match self {
                AVLTree::Node(node) => {
                    (node.right.as_ref().height() as isize) - (node.left.as_ref().height() as isize)
                }
                AVLTree::Nil => 0,
            }
        }
    }
}

impl<K, V> AVLTree<K, V>
where
    K: Ord + Copy,
{
    pub fn get(&self, k: &K) -> Option<&V> {
        match self {
            AVLTree::Node(node) => unsafe {
                match k.cmp(&node.key) {
                    Ordering::Equal => Some(&node.value),
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
                    match k.cmp(&node.key) {
                        Ordering::Less => node.left.as_mut().insert(k, v),
                        Ordering::Greater => node.right.as_mut().insert(k, v),
                        Ordering::Equal => {}
                    }

                    let left_ref = node.left.as_mut();
                    let right_ref = node.right.as_mut();

                    self.reset_height();

                    match self.balance() {
                        -2 => match k.cmp(&left_ref.node().unwrap().key) {
                            Ordering::Less => self.unsafe_rotate_right(),
                            Ordering::Greater => {
                                left_ref.unsafe_rotate_left();
                                self.unsafe_rotate_right();
                            }
                            Ordering::Equal => panic!("can never have balanced"),
                        },
                        2 => match k.cmp(&right_ref.node().unwrap().key) {
                            Ordering::Less => {
                                right_ref.unsafe_rotate_right();
                                self.unsafe_rotate_left();
                            }
                            Ordering::Greater => self.unsafe_rotate_left(),
                            Ordering::Equal => panic!("can never have balanced"),
                        },
                        _ => {}
                    }
                }
                AVLTree::Nil => {
                    let left_ptr = Box::into_raw(Box::new(AVLTree::<K, V>::new()));
                    let right_ptr = Box::into_raw(Box::new(AVLTree::<K, V>::new()));
                    let node = Node {
                        key: k,
                        value: v,
                        left: NonNull::new_unchecked(left_ptr),
                        right: NonNull::new_unchecked(right_ptr),
                        height_m: 1,
                    };
                    *self = AVLTree::Node(node);
                }
            }
        }
    }

    unsafe fn unsafe_rotate_right(&mut self) {
        // Get reference to old root's left child (who will be the new root)
        let mut a = self.node().unwrap().left;
        // Get reference to new root's right child
        let mut b = a.as_ref().node().unwrap().right;
        // Swap out the new root's right child (b will become Nil)
        let mut c = std::mem::take(b.as_mut());
        // Swap new root old child (c) with the old root's left child (a). c is now the new root
        std::mem::swap(&mut c, a.as_mut());
        // Swap new root (c) with the old root (self)
        std::mem::swap(&mut c, self);
        // Set old root (c) to right child of new root (b)
        std::mem::swap(&mut c, b.as_mut());
        // Height of new root and old root has changed
        b.as_mut().reset_height();
        self.reset_height();
    }

    unsafe fn unsafe_rotate_left(&mut self) {
        // Get reference to old root's left child (who will be the new root)
        let mut a = self.node().unwrap().right;
        // Get reference to new root's right child
        let mut b = a.as_ref().node().unwrap().left;
        // Swap out the new root's right child (b will become Nil)
        let mut c = std::mem::take(b.as_mut());
        // Swap new root old child (c) with the old root's left child (a). c is now the new root
        std::mem::swap(&mut c, a.as_mut());
        // Swap new root (c) with the old root (self)
        std::mem::swap(&mut c, self);
        // Set old root (c) to right child of new root (b)
        std::mem::swap(&mut c, b.as_mut());
        // Height of new root and old root has changed
        b.as_mut().reset_height();
        self.reset_height();
    }
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
    key: K,
    value: V,
    left: NonNull<AVLTree<K, V>>,
    right: NonNull<AVLTree<K, V>>,
    height_m: usize,
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
    fn right_rotation() {
        test_insertion_balance(vec![10, 5, 0]);
        test_insertion_balance(vec![15, 10, 20, 5, 0]);
        test_insertion_balance(vec![15, 10, 20, 5, 12, 0]);
    }

    #[test]
    fn left_rotation() {
        test_insertion_balance(vec![0, 5, 10]);
        test_insertion_balance(vec![15, 10, 20, 25, 30]);
        test_insertion_balance(vec![15, 10, 20, 18, 25, 30]);
        test_insertion_balance(vec![15, 10, 20, 18, 25, 22]);
    }

    #[test]
    fn right_left_rotation() {
        test_insertion_balance(vec![15, 10, 20, 18, 25, 19]);
    }

    #[test]
    fn left_right_rotation() {
        test_insertion_balance(vec![15, 10, 20, 5, 12, 14]);
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
