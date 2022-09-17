use std::{cmp::Ordering, collections::VecDeque};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Heap<A> {
    inner: VecDeque<A>,
}

impl<A> Heap<A> {
    pub fn new() -> Self {
        Heap {
            inner: VecDeque::new(),
        }
    }
}

impl<A> Heap<A>
where
    A: Ord,
{
    pub fn size(&self) -> usize {
        self.inner.len()
    }

    pub fn pop(&mut self) -> Option<A> {
        let head = self.inner.swap_remove_back(0);
        if head.is_some() {
            self.sift_down();
        }
        head
    }

    pub fn push(&mut self, a: A) {
        self.inner.push_back(a);
        self.sift_up();
    }

    fn sift_down(&mut self) {
        if self.inner.len() <= 1 {
            return;
        }

        let mut index = 0;
        loop {
            let mut lowest = self.inner.get(index).unwrap();
            let mut new_index = index;
            let first_child = 2 * index + 1;
            let second_child = 2 * index + 1;
            if let Some(value) = self.inner.get(first_child) {
                if value.cmp(lowest) == Ordering::Less {
                    lowest = value;
                    new_index = first_child;
                }
            }
            if let Some(value) = self.inner.get(second_child) {
                if value.cmp(lowest) == Ordering::Less {
                    new_index = second_child;
                }
            }

            if new_index != index {
                self.inner.swap(new_index, index);
                index = new_index;
            } else {
                break;
            }
        }
    }

    fn sift_up(&mut self) {
        let len = self.inner.len();
        if len <= 1 {
            return;
        }

        let mut index = len - 1;
        loop {
            let current = self.inner.get(index).unwrap();
            let mut new_index = index;
            let parent = index / 2;
            if let Some(value) = self.inner.get(parent) {
                if current.cmp(value) == Ordering::Less {
                    new_index = parent;
                }
            }

            if new_index != index {
                self.inner.swap(new_index, index);
                index = new_index;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Heap;

    #[test]
    fn push_and_pop() {
        let mut heap = Heap::new();
        heap.push(3);
        heap.push(2);
        heap.push(1);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn pop_empty() {
        let mut heap: Heap<i32> = Heap::new();
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn pop_single() {
        let mut heap: Heap<i32> = Heap::new();
        heap.push(1);
        assert_eq!(heap.pop(), Some(1));
    }

    #[test]
    fn size() {
        let mut heap = Heap::new();
        assert_eq!(heap.size(), 0);
        heap.push(3);
        heap.push(2);
        assert_eq!(heap.size(), 2);
        heap.pop();
        assert_eq!(heap.size(), 1);
    }

    ///////////////////////
    // PRIVATE API TESTS //
    ///////////////////////

    #[test]
    fn sift_up_idempotence() {
        let mut heap = Heap::new();
        heap.push(3);
        heap.push(2);
        let mut h2 = heap.clone();
        h2.sift_up();
        assert_eq!(heap, h2);
    }

    #[test]
    fn sift_down_idempotence() {
        let mut heap = Heap::new();
        heap.push(3);
        heap.push(2);
        heap.pop();
        let mut h2 = heap.clone();
        h2.sift_down();
        assert_eq!(heap, h2);
    }
}
