



pub struct XorShiftL {
    x: u64,
}

impl XorShiftL {
    pub fn new() -> Self {
        Self { x: 88172645463325252, }
    }

    pub fn next(&mut self) -> u64 {
        self.x = self.x ^ (self.x << 7);
        self.x
    }
}
