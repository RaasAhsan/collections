use std::{cmp::Ordering, collections::VecDeque};

pub struct Heap<A> {
    array: VecDeque<A>,
}

impl<A> Heap<A> {
    pub fn new() -> Self {
        Heap {
            array: VecDeque::new(),
        }
    }
}

impl<A> Heap<A>
where
    A: Ord,
{
    pub fn pop(&mut self) -> Option<A> {
        let head = self.array.swap_remove_back(0);
        if head.is_some() {
            self.sift_down();
        }
        head
    }

    pub fn push(&mut self, a: A) {
        self.array.push_back(a);
        self.sift_up();
    }

    fn sift_down(&mut self) {
        if self.array.len() <= 1 {
            return;
        }

        let mut index = 0;
        loop {
            let mut lowest = self.array.get(index).unwrap();
            let mut new_index = index;
            let first_child = 2 * index + 1;
            let second_child = 2 * index + 1;
            if let Some(value) = self.array.get(first_child) {
                if value.cmp(lowest) == Ordering::Less {
                    lowest = value;
                    new_index = first_child;
                }
            }
            if let Some(value) = self.array.get(second_child) {
                if value.cmp(lowest) == Ordering::Less {
                    new_index = second_child;
                }
            }

            if new_index != index {
                self.array.swap(new_index, index);
                index = new_index;
            } else {
                break;
            }
        }
    }

    fn sift_up(&mut self) {
        let len = self.array.len();
        if len <= 1 {
            return;
        }

        let mut index = len - 1;
        loop {
            let current = self.array.get(index).unwrap();
            let mut new_index = index;
            let parent = index / 2;
            if let Some(value) = self.array.get(parent) {
                if current.cmp(value) == Ordering::Less {
                    new_index = parent;
                }
            }

            if new_index != index {
                self.array.swap(new_index, index);
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
}
