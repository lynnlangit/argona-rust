use crate::hashing::{fast_int_hash, mix_hash};
use core::mem;

const MISSING_VALUE: i32 = i32::MIN;
const MIN_CAPACITY: usize = 8;
const DEFAULT_LOAD_FACTOR: f32 = 0.67;

pub struct IntHashMap<V> {
    keys: Vec<i32>,
    values: Vec<V>,
    size: usize,
    resize_threshold: usize,
    mask: usize,
}

impl<V: Clone + Default> IntHashMap<V> {
    pub fn new() -> Self {
        Self::with_capacity(MIN_CAPACITY)
    }

    pub fn with_capacity(initial_capacity: usize) -> Self {
        let capacity = (initial_capacity.max(MIN_CAPACITY)).next_power_of_two();
        let resize_threshold = (capacity as f32 * DEFAULT_LOAD_FACTOR) as usize;

        Self {
            keys: vec![MISSING_VALUE; capacity],
            values: vec![V::default(); capacity],
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

    pub fn get(&self, key: i32) -> Option<&V> {
        let (index, found) = self.find_index(key);
        if found {
            Some(&self.values[index])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, key: i32) -> Option<&mut V> {
        let (index, found) = self.find_index(key);
        if found {
            Some(&mut self.values[index])
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: i32, value: V) -> Option<V> {
        if self.size >= self.resize_threshold {
            self.resize();
        }

        let (index, found) = self.find_index(key);

        if found {
            let old_value = mem::replace(&mut self.values[index], value);
            Some(old_value)
        } else {
            self.keys[index] = key;
            self.values[index] = value;
            self.size += 1;
            None
        }
    }

    pub fn remove(&mut self, key: i32) -> Option<V> {
        let (index, found) = self.find_index(key);

        if !found {
            return None;
        }

        let old_value = mem::take(&mut self.values[index]);
        self.keys[index] = MISSING_VALUE;
        self.size -= 1;

        self.compact_chain(index);

        Some(old_value)
    }

    pub fn contains_key(&self, key: i32) -> bool {
        self.find_index(key).1
    }

    pub fn clear(&mut self) {
        self.keys.fill(MISSING_VALUE);
        for value in &mut self.values {
            *value = V::default();
        }
        self.size = 0;
    }

    fn resize(&mut self) {
        let old_keys = mem::take(&mut self.keys);
        let old_values = mem::take(&mut self.values);
        let old_size = self.size;

        let new_capacity = old_keys.len() * 2;
        self.keys = vec![MISSING_VALUE; new_capacity];
        self.values = vec![V::default(); new_capacity];
        self.mask = new_capacity - 1;
        self.resize_threshold = (new_capacity as f32 * DEFAULT_LOAD_FACTOR) as usize;
        self.size = 0;

        for (key, value) in old_keys.into_iter().zip(old_values.into_iter()) {
            if key != MISSING_VALUE {
                self.insert(key, value);
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
                self.values[deleted_index] = mem::take(&mut self.values[index]);
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

    pub fn iter(&self) -> IntHashMapIter<V> {
        IntHashMapIter {
            map: self,
            index: 0,
        }
    }

    pub fn keys(&self) -> IntHashMapKeys<V> {
        IntHashMapKeys {
            map: self,
            index: 0,
        }
    }

    pub fn values(&self) -> IntHashMapValues<V> {
        IntHashMapValues {
            map: self,
            index: 0,
        }
    }
}

impl<V: Clone + Default> Default for IntHashMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IntHashMapIter<'a, V> {
    map: &'a IntHashMap<V>,
    index: usize,
}

impl<'a, V> Iterator for IntHashMapIter<'a, V> {
    type Item = (i32, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.map.keys.len() {
            let key = self.map.keys[self.index];
            if key != MISSING_VALUE {
                let value = &self.map.values[self.index];
                self.index += 1;
                return Some((key, value));
            }
            self.index += 1;
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.map.size))
    }
}

pub struct IntHashMapKeys<'a, V> {
    map: &'a IntHashMap<V>,
    index: usize,
}

impl<'a, V> Iterator for IntHashMapKeys<'a, V> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.map.keys.len() {
            let key = self.map.keys[self.index];
            if key != MISSING_VALUE {
                self.index += 1;
                return Some(key);
            }
            self.index += 1;
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.map.size))
    }
}

pub struct IntHashMapValues<'a, V> {
    map: &'a IntHashMap<V>,
    index: usize,
}

impl<'a, V> Iterator for IntHashMapValues<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.map.keys.len() {
            let key = self.map.keys[self.index];
            if key != MISSING_VALUE {
                let value = &self.map.values[self.index];
                self.index += 1;
                return Some(value);
            }
            self.index += 1;
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.map.size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut map = IntHashMap::new();

        assert_eq!(map.len(), 0);
        assert!(map.is_empty());

        assert_eq!(map.insert(1, "one".to_string()), None);
        assert_eq!(map.insert(2, "two".to_string()), None);
        assert_eq!(map.len(), 2);

        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), None);

        assert_eq!(map.insert(1, "ONE".to_string()), Some("one".to_string()));
        assert_eq!(map.len(), 2);

        assert_eq!(map.remove(&1), Some("ONE".to_string()));
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&1), None);
    }

    #[test]
    fn test_resize() {
        let mut map = IntHashMap::with_capacity(4);

        for i in 0..10 {
            map.insert(i, i * 2);
        }

        assert_eq!(map.len(), 10);
        for i in 0..10 {
            assert_eq!(map.get(&i), Some(&(i * 2)));
        }
    }

    #[test]
    fn test_iterators() {
        let mut map = IntHashMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);

        let mut keys: Vec<_> = map.keys().collect();
        keys.sort();
        assert_eq!(keys, vec![1, 2, 3]);

        let mut values: Vec<_> = map.values().cloned().collect();
        values.sort();
        assert_eq!(values, vec![10, 20, 30]);

        let count = map.iter().count();
        assert_eq!(count, 3);
    }
}