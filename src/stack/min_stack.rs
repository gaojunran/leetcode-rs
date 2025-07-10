use std::i32;

struct MinStack {
    vec: Vec<i32>,
    aux: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            vec: Vec::new(),
            aux: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.vec.push(val);
        let top = self.get_min();
        self.aux.push(val.min(top));
    }

    fn pop(&mut self) {
        self.vec.pop();
        self.aux.pop();
    }

    fn top(&self) -> i32 {
        *self.vec.last().unwrap_or(&i32::MAX)
    }

    fn get_min(&self) -> i32 {
        *self.aux.last().unwrap_or(&i32::MAX)
    }
}
