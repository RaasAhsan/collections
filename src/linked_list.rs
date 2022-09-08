use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::{Rc, Weak},
};

/// A doubly linked list which support constant time head insertion, tail deletion, and random deletion.
#[derive(Debug)]
pub struct LinkedList<K> {
    head: Option<Rc<Node<K>>>,
    tail: Option<Rc<Node<K>>>,
}

impl<K> LinkedList<K> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    pub fn push_head(&mut self, k: K) -> LinkedListHandle<K> {
        if let Some(old_head) = self.head.take() {
            let new_head = Rc::new(Node(
                k,
                RefCell::new(None),
                RefCell::new(Some(old_head.clone())),
            ));
            *old_head.1.borrow_mut() = Some(new_head.clone());
            self.head = Some(new_head.clone());
            LinkedListHandle(Rc::downgrade(&new_head))
        } else {
            let new_head = Rc::new(Node(k, RefCell::new(None), RefCell::new(None)));
            self.head = Some(new_head.clone());
            self.tail = Some(new_head.clone());
            LinkedListHandle(Rc::downgrade(&new_head))
        }
    }

    pub fn pop_tail(&mut self) -> Option<K> {
        if let Some(old_tail) = self.tail.take() {
            if Rc::ptr_eq(self.head.borrow().as_ref().unwrap(), &old_tail) {
                self.head.take();
            } else {
                let next_tail = old_tail.1.take().unwrap();
                *next_tail.2.borrow_mut() = None;
                self.tail = Some(next_tail);
            }
            // We should have the only remaining strong reference to this node now,
            // since head, tail, and parent are cleared out
            Some(Rc::try_unwrap(old_tail).ok().unwrap().0)
        } else {
            None
        }
    }

    pub fn remove(&mut self, handle: LinkedListHandle<K>) {
        let mut upgraded = handle.0.upgrade().unwrap();
        let curr = upgraded.borrow_mut();
        let prev = curr.1.take();
        let next = curr.2.take();
        if Rc::ptr_eq(self.head.as_ref().unwrap(), &upgraded) {
            self.head = next.clone();
        } else {
            *prev.borrow().as_ref().unwrap().2.borrow_mut() = next.clone();
        }
        if Rc::ptr_eq(self.tail.as_ref().unwrap(), &upgraded) {
            self.tail = prev.clone();
        } else {
            *next.borrow().as_ref().unwrap().1.borrow_mut() = prev.clone();
        }
    }
}

/// A handle to a particular node in a LinkedList. This is useful for
/// random deletions. This handle will be rendered stale if the referenced
/// node is deleted from the list.
#[derive(Debug)]
pub struct LinkedListHandle<K>(Weak<Node<K>>);

#[derive(Debug)]
struct Node<K>(
    K,
    RefCell<Option<Rc<Node<K>>>>,
    RefCell<Option<Rc<Node<K>>>>,
);
