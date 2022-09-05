use std::{cell::RefCell, collections::HashMap, fmt::Debug, hash::Hash, rc::Rc};

#[derive(Debug)]
pub struct LRUCache<K, V> {
    entries: HashMap<K, V>,
    recent: HashMap<K, NodeHandle<K>>,
    list: List<K>,
    len: usize,
    capacity: usize,
}

impl<K, V> LRUCache<K, V>
where
    K: Clone,
{
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            entries: HashMap::new(),
            recent: HashMap::new(),
            list: List::new(),
            len: 0,
            capacity,
        }
    }
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn insert(&mut self, k: K, v: V) {
        if let Some(value) = self.entries.get_mut(&k) {
            *value = v;
            return;
        }

        if self.len < self.capacity {
            self.len += 1;
        } else {
            let removed = self.list.pop_tail().unwrap();
            self.recent.remove(&removed);
            self.entries.remove(&removed);
        }

        let handle = self.list.push_head(k.clone());
        self.recent.insert(k.clone(), handle);

        self.entries.insert(k, v);
    }

    pub fn get(&mut self, k: &K) -> Option<&V> {
        let handle = self.recent.remove(k);
        if let Some(handle) = handle {
            self.list.remove(handle);
        }
        let new_handle = self.list.push_head(k.clone());
        self.recent.insert(k.clone(), new_handle);
        self.entries.get(&k)
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.entries.get_mut(&k)
    }
}

#[derive(Debug)]
struct List<K> {
    head: Option<Rc<RefCell<Node<K>>>>,
    tail: Option<Rc<RefCell<Node<K>>>>,
}

impl<K> List<K>
where
    K: Clone,
{
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_head(&mut self, k: K) -> NodeHandle<K> {
        if let Some(old_head) = self.head.take() {
            let new_head = Rc::new(RefCell::new(Node(k, None, Some(old_head.clone()))));
            old_head.borrow_mut().1 = Some(new_head.clone());
            self.head = Some(new_head.clone());
            NodeHandle(new_head)
        } else {
            let new_head = Rc::new(RefCell::new(Node(k, None, None)));
            self.head = Some(new_head.clone());
            self.tail = Some(new_head.clone());
            NodeHandle(new_head)
        }
    }

    pub fn pop_tail(&mut self) -> Option<K> {
        if let Some(old_tail) = self.tail.take() {
            if Rc::ptr_eq(self.head.as_ref().unwrap(), &old_tail) {
                self.head.take();
            } else {
                self.tail = old_tail.borrow_mut().1.clone();
            }

            Some(old_tail.borrow_mut().0.clone())
        } else {
            None
        }
    }

    pub fn remove(&mut self, handle: NodeHandle<K>) {
        let mut curr = handle.0.borrow_mut();
        let prev = curr.1.take();
        let next = curr.2.take();
        if Rc::ptr_eq(self.head.as_ref().unwrap(), &handle.0) {
            self.head = next.clone();
        } else {
            prev.as_ref().unwrap().borrow_mut().2 = next.clone();
        }
        if Rc::ptr_eq(self.tail.as_ref().unwrap(), &handle.0) {
            self.tail = prev.clone();
        } else {
            next.as_ref().unwrap().borrow_mut().1 = prev.clone();
        }
    }
}

#[derive(Debug)]
struct NodeHandle<K>(Rc<RefCell<Node<K>>>);

#[derive(Debug)]
struct Node<K>(
    K,
    Option<Rc<RefCell<Node<K>>>>,
    Option<Rc<RefCell<Node<K>>>>,
);

#[cfg(test)]
mod test {
    use super::LRUCache;

    #[test]
    fn cache_retrieve() {
        let mut cache = LRUCache::new(2);
        cache.insert(1, 100);
        assert_eq!(cache.get(&1), Some(&100));
    }

    #[test]
    fn cache_evict() {
        let mut cache = LRUCache::new(2);
        cache.insert(1, 101);
        cache.insert(2, 102);
        cache.insert(3, 103);
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&102));
        assert_eq!(cache.get(&3), Some(&103));
    }

    #[test]
    fn cache_recent() {
        let mut cache = LRUCache::new(2);
        cache.insert(1, 101);
        cache.insert(2, 102);
        cache.get(&1);
        cache.insert(3, 103);
        assert_eq!(cache.get(&1), Some(&101));
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some(&103));
    }
}
