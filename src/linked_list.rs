use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::{Rc, Weak},
};

/// A doubly linked list which support constant time head insertion, tail deletion, and random deletion.
#[derive(Debug, Default)]
pub struct LinkedList<A> {
    head: Option<Rc<Node<A>>>,
    tail: Option<Rc<Node<A>>>,
}

impl<A> LinkedList<A> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn push_head(&mut self, k: A) -> LinkedListHandle<A> {
        if let Some(old_head) = self.head.take() {
            let new_head = Rc::new(Node::new(k, None, Some(old_head.clone())));
            *old_head.prev.borrow_mut() = Some(new_head.clone());
            self.head = Some(new_head.clone());
            LinkedListHandle(Rc::downgrade(&new_head))
        } else {
            let new_head = Rc::new(Node::new(k, None, None));
            self.head = Some(new_head.clone());
            self.tail = Some(new_head.clone());
            LinkedListHandle(Rc::downgrade(&new_head))
        }
    }

    pub fn pop_tail(&mut self) -> Option<A> {
        if let Some(old_tail) = self.tail.take() {
            if Rc::ptr_eq(self.head.borrow().as_ref().unwrap(), &old_tail) {
                self.head.take();
            } else {
                let next_tail = old_tail.prev.take().unwrap();
                *next_tail.next.borrow_mut() = None;
                self.tail = Some(next_tail);
            }
            // We should have the only remaining strong reference to this node now,
            // since head, tail, and parent are cleared out
            Some(Rc::try_unwrap(old_tail).ok().unwrap().key)
        } else {
            None
        }
    }

    pub fn remove(&mut self, handle: LinkedListHandle<A>) {
        let mut upgraded = handle.0.upgrade().unwrap();
        let curr = upgraded.borrow_mut();
        let prev = curr.prev.take();
        let next = curr.next.take();
        if Rc::ptr_eq(self.head.as_ref().unwrap(), &upgraded) {
            self.head = next.clone();
        } else {
            *prev.borrow().as_ref().unwrap().next.borrow_mut() = next.clone();
        }
        if Rc::ptr_eq(self.tail.as_ref().unwrap(), &upgraded) {
            self.tail = prev;
        } else {
            *next.borrow().as_ref().unwrap().prev.borrow_mut() = prev;
        }
    }

    // pub fn iter<'a>(&'a self) -> Iter<'a, A> {
    //     Iter { head: self.head.as_ref().map(|n| n.as_ref()), tail: self.tail.as_ref().map(|n| n.as_ref()) }
    // }
}

// pub struct Iter<A> {
//     head: Option<AsRef<Node<A>>,
//     tail: Option<AsRef<Node<A>>,
// }

// impl<A> Iterator for Iter<A> {
//     type Item = &A;

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(head) = self.head.take() {
//             let item = Ref::map(head, |n| &n.key);

//             // Invariant: if there is a head, there must be a tail
//             let tail = self.tail.unwrap();
//             if std::ptr::eq(head, tail) {
//                 self.head = None;
//                 self.tail = None;
//             } else {
//                 // We have a tail element next
//                 let next_head = head.next.borrow();
//                 let x = Ref::map(next_head, |n| &n.unwrap());
//                 self.head = Some(next_head);
//             }
//             Some(item)
//         } else {
//             None
//         }
//     }
// }

// impl<'a, A> DoubleEndedIterator for Iter<'a, A> {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

/// A handle to a particular node in a LinkedList. This is useful for
/// random deletions. This handle will be rendered stale if the referenced
/// node is deleted from the list.
#[derive(Debug)]
pub struct LinkedListHandle<K>(Weak<Node<K>>);

#[derive(Debug)]
struct Node<K> {
    key: K,
    prev: RefCell<Option<Rc<Node<K>>>>,
    next: RefCell<Option<Rc<Node<K>>>>,
}

impl<K> Node<K> {
    pub fn new(key: K, prev: Option<Rc<Node<K>>>, next: Option<Rc<Node<K>>>) -> Self {
        Node {
            key,
            prev: RefCell::new(prev),
            next: RefCell::new(next),
        }
    }
}
