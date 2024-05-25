use anyhow::bail;
pub struct BufferPool {
    inner: Vec<Vec<u8>>,
    buffs: u16,
    length: u32,
    count: u16
}

impl BufferPool {
    pub fn new(buffer_count: u16, buffer_size: u32) -> Self {
        BufferPool {
            inner: vec![vec![0u8; buffer_size as usize]; buffer_count as usize],
            buffs: buffer_count,
            length: buffer_size,
            count: buffer_count,
        }
    }
    /// Returns a buffer from the pool 
    /// If there are no buffers left it will create one
    pub fn get(&mut self) -> Vec<u8> {
        if let Some(buff) = self.inner.pop() {
            self.count -= 1;
            return buff
        } else {
            return vec![0u8; self.length as usize] 
        }
    }

    /// Returns the buffer back to the pool
    /// It is a good idea **NOT** to change the size of the buffers
    pub fn give_back(&mut self, buff: Vec<u8>) {
        self.count += 1;
        self.inner.push(buff);
    }


    /// Returns the buffer count in the pool
    pub fn count(&self) -> u16 {
        self.count
    }
}
