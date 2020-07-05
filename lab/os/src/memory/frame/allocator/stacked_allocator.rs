use alloc::{vec, vec::Vec};

use super::Allocator;

pub struct StackedAllocator {
    list: Vec<(usize, usize)>
}

impl Allocator for StackedAllocator {
    fn new(capacity: usize) -> Self {
        Self {
            list: vec![(0, capacity)],
        }
    }

    fn alloc(&mut self) -> Option<usize> {
        if let Some((start, end)) = self.list.pop() {
            if end - start > 1 {
                self.list.push((start + 1, end));
            }
            return Some(start);
        }
        None
    }

    fn dealloc(&mut self, index: usize) {
        self.list.push((index, index + 1))
    }
}