use anyhow::bail;
pub struct BufferPool {
    inner: Vec<Vec<u8>>,
<<<<<<< HEAD
=======
    buffs: u16,
    length: u32,
    count: u16
>>>>>>> 2ec516f (Buffer Pooling & Creating of packets)
}

impl BufferPool {
    pub fn new(buffer_count: u16, buffer_size: u32) -> Self {
        BufferPool {
            inner: vec![vec![0u8; buffer_size as usize]; buffer_count as usize],
<<<<<<< HEAD
        }
    }
    /// Returns a buffer from the pool 
    pub fn get(&mut self) -> Option<Vec<u8>> {
       self.inner.pop()
=======
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
>>>>>>> 2ec516f (Buffer Pooling & Creating of packets)
    }

    /// Returns the buffer back to the pool
    /// It is a good idea **NOT** to change the size of the buffers
    pub fn give_back(&mut self, buff: Vec<u8>) {
<<<<<<< HEAD
        self.inner.push(buff);
    }

    /// Creates a new buffer in the pool with the size provided
    /// Use with caution doing this operation many times can lead 
    /// to performance issues. 
    ///
    /// You can also use this to create a more dynamic buffer pool.
    /// By allocating 0 buffers you can create buffers only when you need to and
    /// giving them to the pool
    /// This will give you less allocations
    pub fn create(&mut self, size: u32) {
        self.inner.push(vec![0u8;size as usize]);
    }

    /// Returns the buffer count in the pool
    pub fn count(&self) -> usize {
        self.inner.len()
=======
        self.count += 1;
        self.inner.push(buff);
    }


    /// Returns the buffer count in the pool
    pub fn count(&self) -> u16 {
        self.count
>>>>>>> 2ec516f (Buffer Pooling & Creating of packets)
    }
}
