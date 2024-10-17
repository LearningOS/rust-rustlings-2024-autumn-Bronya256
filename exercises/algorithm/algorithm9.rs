
/*
    Heap
*/
use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        self.heapify_up(self.count);
    }

    fn parent_idx(idx: usize) -> usize {
        idx / 2
    }

    fn left_child_idx(idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(idx: usize) -> usize {
        idx * 2 + 1
    }

    fn heapify_up(&mut self, mut idx: usize) {
        while idx > 1 && (self.comparator)(&self.items[idx], &self.items[Self::parent_idx(idx)]) {
            let parent_idx = Self::parent_idx(idx);
            self.items.swap(idx, parent_idx);
            idx = parent_idx;
        }
    }

    fn heapify_down(&mut self, mut idx: usize) {
        while Self::left_child_idx(idx) <= self.count {
            let mut child_idx = Self::left_child_idx(idx);
            if child_idx < self.count && (self.comparator)(&self.items[child_idx + 1], &self.items[child_idx]) {
                child_idx += 1;
            }
            if !(self.comparator)(&self.items[child_idx], &self.items[idx]) {
                break;
            }
            self.items.swap(idx, child_idx);
            idx = child_idx;
        }
    }

    fn remove(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.items.swap(1, self.count);
        let removed = self.items.pop();
        self.count -= 1;
        if !self.is_empty() {
            self.heapify_down(1);
        }
        removed
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.remove()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
