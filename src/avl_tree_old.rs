// use std::{cell::RefCell, rc::Rc, ops::DerefMut};

// /// An AVL tree is a self-balancing binary search tree.
// /// Invariant: for any node N in the tree, the heights of both its children
// /// may not differ by more than 1.
// #[derive(Debug)]
// pub enum AVLTree<K, V> {
//     Node {
//         key: K,
//         value: V,
//         left: Rc<RefCell<AVLTree<K, V>>>,
//         right: Rc<RefCell<AVLTree<K, V>>>,
//         node_height: usize
//     },
//     Nil
// }

// impl<K, V> Default for AVLTree<K, V> {
//     fn default() -> Self {
//         AVLTree::Nil
//     }
// }

// impl<K, V> AVLTree<K, V> {
//     pub fn new() -> Self {
//         Self::nil()
//     }

//     fn nil() -> Self {
//         AVLTree::Nil
//     }

//     fn node(key: K, value: V) -> Self {
//         AVLTree::Node {
//             key,
//             value,
//             left: Rc::new(RefCell::new(AVLTree::Nil)),
//             right: Rc::new(RefCell::new(AVLTree::Nil)),
//             node_height: 1
//         }
//     }

//     fn is_nil(&self) -> bool {
//         match self {
//             AVLTree::Node { key, value, left, right, node_height: height } => false,
//             AVLTree::Nil => true,
//         }
//     }

//     fn left(&self) -> Option<Rc<RefCell<AVLTree<K, V>>>> {
//         match self {
//             AVLTree::Node { key, value, left, right, node_height: height } => {
//                 Some(left.clone())
//             },
//             AVLTree::Nil => None,
//         }
//     }

//     fn right(&mut self) -> Option<Rc<RefCell<AVLTree<K, V>>>> {
//         match self {
//             AVLTree::Node { key, value, left, right, node_height: height } => {
//                 Some(right.clone())
//             },
//             AVLTree::Nil => None,
//         }
//     }

//     pub fn height(&self) -> usize {
//         match self {
//             AVLTree::Node { key, value, left, right, node_height } => *node_height,
//             AVLTree::Nil => 0,
//         }
//     }

//     pub fn reset_height(&mut self) {
//         match self {
//             AVLTree::Node { key, value, left, right, node_height } => {
//                 *node_height = 1 + std::cmp::max(left.borrow().height(), right.borrow().height());
//             },
//             AVLTree::Nil => {},
//         }
//     }

//     pub fn balance(&self) -> isize {
//         match self {
//             AVLTree::Node { key, value, left, right, node_height } => {
//                 (right.borrow().height() as isize) - (left.borrow().height() as isize)
//             },
//             AVLTree::Nil => 0,
//         }
//     }
// }

// impl<K, V> AVLTree<K, V> where K: Ord + Copy, V: Copy {
//     pub fn get(&self, key: &K) -> Option<V> {
//         match self {
//             AVLTree::Node { key: nkey, value, left, right, node_height: _ } => {
//                 match key.cmp(nkey) {
//                     std::cmp::Ordering::Less => left.borrow().get(key),
//                     std::cmp::Ordering::Greater => right.borrow().get(key),
//                     std::cmp::Ordering::Equal => Some(*value),
//                 }
//             },
//             AVLTree::Nil => None,
//         }
//     }

//     pub fn insert(&mut self, key: K, value: V) {
//         match self {
//             AVLTree::Node { key: nkey, value: _, left, right, node_height } => {
//                 match key.cmp(nkey) {
//                     std::cmp::Ordering::Less => left.borrow_mut().insert(key, value),
//                     std::cmp::Ordering::Greater => right.borrow_mut().insert(key, value),
//                     std::cmp::Ordering::Equal => {}, // TODO: replace?
//                 }

//                 let left_clone = left.clone();

//                 self.reset_height();
//                 let balance = self.balance();

//                 match balance {
//                     2 => {},
//                     -2 => {
//                         let side = left_clone.borrow().compare(&key).unwrap();
//                         match side {
//                             std::cmp::Ordering::Less => self.right_rotate(left_clone),
//                             std::cmp::Ordering::Equal => panic!("Unexpected"),
//                             std::cmp::Ordering::Greater => todo!(),
//                         }
//                     },
//                     -1 | 0 | 1 => {},
//                     _ => panic!("unexpected balance factor")
//                 }
//             },
//             AVLTree::Nil => *self = Self::node(key, value),
//         }
//     }

//     fn right_rotate(&mut self, left_child: Rc<RefCell<AVLTree<K, V>>>) {
//         let mut left_borrow = left_child.borrow_mut();
//         let left = left_borrow.deref_mut();
//         // Detach left child from tree
//         let mut new_root = std::mem::take(left);
//         // Attach right child of new root to left child of old root(self)
//         std::mem::swap(new_root.right().unwrap().borrow_mut().deref_mut(), left);
//         // Swap old root (self) to next root's left
//         let mut old_root = std::mem::take(self);
//         std::mem::swap(self, &mut new_root);
//         // Finally, move old root into left root's right child
//         {
//             let right_borrow = self.right().unwrap();
//             let mut right = right_borrow.borrow_mut();
//             std::mem::swap(new_root_right, &mut old_root);
//             new_root_right.reset_height();
//         }

//         new_root.reset_height();
//     }

//     pub fn compare(&self, k: &K) -> Option<std::cmp::Ordering> {
//         match self {
//             AVLTree::Node { key, value, left, right, node_height } => {
//                 Some(k.cmp(key))
//             },
//             AVLTree::Nil => None,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::AVLTree;

//     #[test]
//     fn rebalance_left() {
//         let mut tree = AVLTree::<i32, i32>::new();
//         tree.insert(15, 0);
//         tree.insert(20, 0);
//         tree.insert(10, 0);
//         tree.insert(5, 0);
//         tree.insert(0, 0);
//         dbg!(tree);
//     }
// }
