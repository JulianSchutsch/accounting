pub struct SequenceGenerator<T>  where T: num::Integer {
    pos: T
}

impl<T:num::Integer+std::marker::Copy> SequenceGenerator<T> {
    pub fn new() -> Self {
        Self{ pos: T::zero() }
    }
    pub fn generate(&mut self) -> T {
        let result = self.pos;
        self.pos.inc();
        result
    }
    pub fn reset(&mut self) {
        self.pos = T::zero();
    }
}