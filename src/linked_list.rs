use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct LinkedList<K> {
    head: RefCell<Option<Rc<Node<K>>>>,
    tail: RefCell<Option<Rc<Node<K>>>>,
}

impl<K> LinkedList<K> {
    pub fn new() -> Self {
        LinkedList {
            head: RefCell::new(None),
            tail: RefCell::new(None),
        }
    }

    pub fn push_head(&self, k: K) -> LinkedListHandle<K> {
        if let Some(old_head) = self.head.take() {
            let new_head = Rc::new(Node(
                k,
                RefCell::new(None),
                RefCell::new(Some(old_head.clone())),
            ));
            *old_head.1.borrow_mut() = Some(new_head.clone());
            *self.head.borrow_mut() = Some(new_head.clone());
            LinkedListHandle(Rc::downgrade(&new_head))
        } else {
            let new_head = Rc::new(Node(k, RefCell::new(None), RefCell::new(None)));
            *self.head.borrow_mut() = Some(new_head.clone());
            *self.tail.borrow_mut() = Some(new_head.clone());
            LinkedListHandle(Rc::downgrade(&new_head))
        }
    }

    pub fn pop_tail(&self) -> Option<K> {
        if let Some(old_tail) = self.tail.take() {
            if Rc::ptr_eq(self.head.borrow().as_ref().unwrap(), &old_tail) {
                self.head.take();
            } else {
                let next_tail = old_tail.1.take().unwrap();
                *next_tail.2.borrow_mut() = None;
                *self.tail.borrow_mut() = Some(next_tail);
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
        if Rc::ptr_eq(self.head.borrow().as_ref().unwrap(), &upgraded) {
            *self.head.borrow_mut() = next.clone();
        } else {
            *prev.borrow().as_ref().unwrap().2.borrow_mut() = next.clone();
        }
        if Rc::ptr_eq(&self.tail.borrow().as_ref().unwrap(), &upgraded) {
            *self.tail.borrow_mut() = prev.clone();
        } else {
            *next.borrow().as_ref().unwrap().1.borrow_mut() = prev.clone();
        }
    }
}

#[derive(Debug)]
pub struct LinkedListHandle<K>(Weak<Node<K>>);

#[derive(Debug)]
struct Node<K>(
    K,
    RefCell<Option<Rc<Node<K>>>>,
    RefCell<Option<Rc<Node<K>>>>,
);
