use std::{
    cell::RefCell,
    mem::{size_of, size_of_val},
    ops::Deref,
    rc::Rc,
};

use rastd::{avl_tree::AVLTree, hash_trie::HashTrie, linked_list::LinkedList};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

fn main() {
    // let a = Rc::new(List::Cons(1, RefCell::new(Rc::new(List::Nil))));
    // let b = Rc::new(List::Cons(2, RefCell::new(a.clone())));

    // if let List::Cons(_, tail) = a.deref() {
    //     *tail.borrow_mut() = b;
    // }

    // println!("{}", size_of::<RefCell<Rc<List>>>());
    // println!("{}", size_of::<List>());
    // println!("{}", size_of_val(&List::Nil));

    // let mut list = LinkedList::new();
    // list.push_head(3);
    // list.push_head(3);
    // list.push_head(3);
    // list.push_head(3);
    // dbg!(a);

    let mut tree = AVLTree::<i32, i32>::new();
    tree.insert(15, 0);
    tree.insert(20, 0);
    tree.insert(10, 0);
    dbg!(&tree);
    tree.insert(5, 0);
    tree.insert(0, 0);
    dbg!(&tree);
}
