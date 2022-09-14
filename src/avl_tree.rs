use std::{cmp::Ordering, ptr::NonNull};

#[derive(Debug)]
pub enum AVLTree<K, V> {
    Node {
        key: K,
        value: V,
        left: NonNull<AVLTree<K, V>>,
        right: NonNull<AVLTree<K, V>>,
        m_height: usize,
    },
    Nil,
}

impl<K, V> AVLTree<K, V> {
    pub fn new() -> Self {
        Self::Nil
    }

    fn key(&self) -> Option<&K> {
        match self {
            AVLTree::Node { key, .. } => Some(key),
            AVLTree::Nil => None,
        }
    }

    pub fn height(&self) -> usize {
        match self {
            AVLTree::Node { m_height, .. } => *m_height,
            AVLTree::Nil => 0,
        }
    }

    fn reset_height(&mut self) {
        unsafe {
            match self {
                AVLTree::Node {
                    left,
                    right,
                    m_height,
                    ..
                } => *m_height = 1 + std::cmp::max(left.as_ref().height(), right.as_ref().height()),
                AVLTree::Nil => {}
            }
        }
    }

    pub fn balance(&self) -> isize {
        unsafe {
            match self {
                AVLTree::Node { left, right, .. } => {
                    (right.as_ref().height() as isize) - (left.as_ref().height() as isize)
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
    pub fn insert(&mut self, k: K, v: V) {
        unsafe {
            match self {
                AVLTree::Node {
                    key,
                    value,
                    left,
                    right,
                    m_height,
                } => {
                    match k.cmp(key) {
                        Ordering::Less => left.as_mut().insert(k, v),
                        Ordering::Greater => right.as_mut().insert(k, v),
                        Ordering::Equal => {}
                    }

                    let left_ref = left.as_ref();

                    self.reset_height();

                    match self.balance() {
                        -2 => match k.cmp(left_ref.key().unwrap()) {
                            Ordering::Less => self.unsafe_rotate_right(),
                            Ordering::Greater => {}
                            Ordering::Equal => {}
                        },
                        2 => {}
                        _ => {}
                    }
                }
                AVLTree::Nil => {
                    let left_ptr = Box::into_raw(Box::new(AVLTree::<K, V>::new()));
                    let right_ptr = Box::into_raw(Box::new(AVLTree::<K, V>::new()));
                    *self = AVLTree::Node {
                        key: k,
                        value: v,
                        left: NonNull::new_unchecked(left_ptr),
                        right: NonNull::new_unchecked(right_ptr),
                        m_height: 1,
                    }
                }
            }
        }
    }

    fn left_child(&self) -> Option<NonNull<AVLTree<K, V>>> {
        match self {
            AVLTree::Node { left, .. } => Some(*left),
            AVLTree::Nil => None,
        }
    }

    fn right_child(&self) -> Option<NonNull<AVLTree<K, V>>> {
        match self {
            AVLTree::Node { right, .. } => Some(*right),
            AVLTree::Nil => None,
        }
    }

    unsafe fn unsafe_rotate_right(&mut self) {
        // Get reference to old root's left child (who will be the new root)
        let mut a = self.left_child().unwrap();
        // Get reference to new root's right child
        let mut b = a.as_ref().right_child().unwrap();
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
        let mut a = self.right_child().unwrap();
        // Get reference to new root's right child
        let mut b = a.as_ref().left_child().unwrap();
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

#[cfg(test)]
mod tests {
    use crate::avl_tree::AVLTree;

    impl<K, V> AVLTree<K, V> {
        fn height_internal(&self) -> usize {
            unsafe {
                match self {
                    AVLTree::Node { left, right, .. } => {
                        1 + std::cmp::max(
                            left.as_ref().height_internal(),
                            right.as_ref().height_internal(),
                        )
                    }
                    AVLTree::Nil => 0,
                }
            }
        }

        fn balanced_internal(&self) -> bool {
            unsafe {
                match self {
                    AVLTree::Node { left, right, .. } => {
                        let left = left.as_ref();
                        let right = right.as_ref();
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

    #[test]
    fn tree_balanced() {
        let mut tree = AVLTree::<i32, i32>::new();
        tree.insert(15, 0);
        tree.insert(20, 0);
        tree.insert(10, 0);
        tree.insert(5, 0);
        tree.insert(0, 0);
        assert!(tree.balanced_internal());
    }
}
