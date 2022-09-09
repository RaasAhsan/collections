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

    pub fn pop_tail(&mut self) -> Option<K> {
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

    pub fn remove(&mut self, handle: LinkedListHandle<K>) {
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
}

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
