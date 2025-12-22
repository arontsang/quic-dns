use zeropool::PooledBuffer;

pub struct SlicedBuffer {

    buf: PooledBuffer,
    length: usize,
}

impl SlicedBuffer {
    pub fn new(buf: PooledBuffer, length: usize) -> SlicedBuffer {
        Self { buf, length }
    }
}

impl AsRef<[u8]> for SlicedBuffer {
    fn as_ref(&self) -> &[u8] {
        &self.buf.as_ref()[..self.length]
    }
}