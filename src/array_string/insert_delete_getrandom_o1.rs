use rand::{Rng, thread_rng};
use std::collections::HashMap;

struct RandomizedSet {
    values: Vec<i32>,
    indices: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            values: Vec::new(),
            indices: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.indices.contains_key(&val) {
            return false;
        }
        self.values.push(val);
        self.indices.insert(val, self.values.len() - 1);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&idx) = self.indices.get(&val) {
            let last = *self.values.last().unwrap();
            self.values[idx] = last;
            self.indices.insert(last, idx);
            self.values.pop();
            self.indices.remove(&val);
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        let idx = rng.gen_range(0..self.values.len());
        self.values[idx]
    }
}
