use std::mem::MaybeUninit;
use compio::buf::{IoBuf, IoBufMut, SetLen};

use zeropool::PooledBuffer;

pub struct PooledBufMut {
    buf: PooledBuffer
}



impl PooledBufMut {
    pub fn new(buf: PooledBuffer) -> Self {
        Self{ buf }
    }
}

impl Into<PooledBuffer> for PooledBufMut {
    fn into(self) -> PooledBuffer {
        self.buf
    }
}

impl IoBuf for PooledBufMut {
    fn as_init(&self) -> &[u8] {
        self.buf.as_init()
    }

    fn buf_len(&self) -> usize {
        self.buf.buf_len()
    }

    fn buf_ptr(&self) -> *const u8 {
        self.buf.buf_ptr()
    }
}

impl SetLen for PooledBufMut {
    unsafe fn set_len(&mut self, len: usize) {
        unsafe {
            self.buf.set_len(len)
        }
    }
}

impl IoBufMut for PooledBufMut {
    fn as_uninit(&mut self) -> &mut [MaybeUninit<u8>] {
        self.buf.as_uninit()
    }
}

