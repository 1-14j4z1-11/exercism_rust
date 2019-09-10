pub struct Memory {
    memory: Vec<u8>,
    ptr: usize,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: vec![0],
            ptr: 0,
        }
    }

    #[inline]
    pub fn next(&mut self) {
        self.ptr += 1;
        self.extend_memory();
    }

    #[inline]
    pub fn prev(&mut self) {
        self.ptr -= 1;
    }

    #[inline]
    pub fn increment(&mut self) {
        self.memory[self.ptr] += 1;
    }

    #[inline]
    pub fn decrement(&mut self) {
        self.memory[self.ptr] -= 1;
    }

    #[inline]
    pub fn store(&mut self, value: u8) {
        self.memory[self.ptr] = value;
    }

    #[inline]
    pub fn load(&self) -> u8 {
        self.memory[self.ptr]
    }

    #[inline]
    fn extend_memory(&mut self) {
        for _ in self.memory.len()..=self.ptr {
            self.memory.push(0);
        }
    }
}
