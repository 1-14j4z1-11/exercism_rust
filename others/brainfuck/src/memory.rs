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

    pub fn next(&mut self) {
        self.ptr += 1;
        self.extend_memory();
    }

    pub fn prev(&mut self) {
        self.ptr -= 1;
    }

    pub fn increment(&mut self) {
        self.memory[self.ptr] += 1;
    }

    pub fn decrement(&mut self) {
        self.memory[self.ptr] -= 1;
    }

    pub fn store(&mut self, value: u8) {
        self.memory[self.ptr] = value;
    }

    pub fn load(&self) -> u8 {
        self.memory[self.ptr]
    }

    fn extend_memory(&mut self) {
        for _ in self.memory.len()..=self.ptr {
            self.memory.push(0);
        }
    }
}
