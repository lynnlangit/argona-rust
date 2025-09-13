use crate::hashing::{fast_int_hash, mix_hash};

const MISSING_VALUE: i32 = i32::MIN;
const MIN_CAPACITY: usize = 8;
const DEFAULT_LOAD_FACTOR: f32 = 0.67;

pub struct IntHashSet {
    keys: Vec<i32>,
    size: usize,
    resize_threshold: usize,
    mask: usize,
}

impl IntHashSet {
    pub fn new() -> Self {
        Self::with_capacity(MIN_CAPACITY)
    }

    pub fn with_capacity(initial_capacity: usize) -> Self {
        let capacity = (initial_capacity.max(MIN_CAPACITY)).next_power_of_two();
        let resize_threshold = (capacity as f32 * DEFAULT_LOAD_FACTOR) as usize;

        Self {
            keys: vec![MISSING_VALUE; capacity],
            size: 0,
            resize_threshold,
            mask: capacity - 1,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.keys.len()
    }

    #[inline]
    fn hash_key(key: i32) -> usize {
        mix_hash(fast_int_hash(key)) as usize
    }

    #[inline]
    fn find_index(&self, key: i32) -> (usize, bool) {
        let mut index = Self::hash_key(key) & self.mask;

        loop {
            let existing_key = self.keys[index];
            if existing_key == MISSING_VALUE {
                return (index, false);
            }
            if existing_key == key {
                return (index, true);
            }
            index = (index + 1) & self.mask;
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        self.find_index(key).1
    }

    pub fn insert(&mut self, key: i32) -> bool {
        if self.size >= self.resize_threshold {
            self.resize();
        }

        let (index, found) = self.find_index(key);

        if !found {
            self.keys[index] = key;
            self.size += 1;
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, key: i32) -> bool {
        let (index, found) = self.find_index(key);

        if !found {
            return false;
        }

        self.keys[index] = MISSING_VALUE;
        self.size -= 1;

        self.compact_chain(index);

        true
    }

    pub fn clear(&mut self) {
        self.keys.fill(MISSING_VALUE);
        self.size = 0;
    }

    fn resize(&mut self) {
        let old_keys = std::mem::take(&mut self.keys);
        let old_size = self.size;

        let new_capacity = old_keys.len() * 2;
        self.keys = vec![MISSING_VALUE; new_capacity];
        self.mask = new_capacity - 1;
        self.resize_threshold = (new_capacity as f32 * DEFAULT_LOAD_FACTOR) as usize;
        self.size = 0;

        for key in old_keys {
            if key != MISSING_VALUE {
                self.insert(key);
            }
        }

        debug_assert_eq!(self.size, old_size);
    }

    fn compact_chain(&mut self, deleted_index: usize) {
        let mut index = (deleted_index + 1) & self.mask;

        while self.keys[index] != MISSING_VALUE {
            let key = self.keys[index];
            let ideal_index = Self::hash_key(key) & self.mask;

            if self.should_move_entry(deleted_index, index, ideal_index) {
                self.keys[deleted_index] = key;
                self.keys[index] = MISSING_VALUE;

                self.compact_chain(index);
                break;
            }

            index = (index + 1) & self.mask;
        }
    }

    #[inline]
    fn should_move_entry(&self, deleted_index: usize, current_index: usize, ideal_index: usize) -> bool {
        if deleted_index < current_index {
            ideal_index <= deleted_index || ideal_index > current_index
        } else {
            ideal_index <= deleted_index && ideal_index > current_index
        }
    }

    pub fn iter(&self) -> IntHashSetIter<'_> {
        IntHashSetIter {
            set: self,
            index: 0,
        }
    }
}

impl Default for IntHashSet {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IntHashSetIter<'a> {
    set: &'a IntHashSet,
    index: usize,
}

impl<'a> Iterator for IntHashSetIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.set.keys.len() {
            let key = self.set.keys[self.index];
            if key != MISSING_VALUE {
                self.index += 1;
                return Some(key);
            }
            self.index += 1;
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.set.size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut set = IntHashSet::new();

        assert_eq!(set.len(), 0);
        assert!(set.is_empty());

        assert!(set.insert(1));
        assert!(set.insert(2));
        assert_eq!(set.len(), 2);

        assert!(set.contains(1));
        assert!(set.contains(2));
        assert!(!set.contains(3));

        assert!(!set.insert(1));
        assert_eq!(set.len(), 2);

        assert!(set.remove(1));
        assert_eq!(set.len(), 1);
        assert!(!set.contains(1));
    }

    #[test]
    fn test_resize() {
        let mut set = IntHashSet::with_capacity(4);

        for i in 0..10 {
            set.insert(i);
        }

        assert_eq!(set.len(), 10);
        for i in 0..10 {
            assert!(set.contains(i));
        }
    }

    #[test]
    fn test_iterator() {
        let mut set = IntHashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        let mut values: Vec<_> = set.iter().collect();
        values.sort();
        assert_eq!(values, vec![1, 2, 3]);
    }
}