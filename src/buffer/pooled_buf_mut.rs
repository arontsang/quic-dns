
use compio::buf::{IoBuf, IoBufMut, IoSliceMut, SetBufInit};
use std::mem::MaybeUninit;
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



unsafe impl IoBuf for PooledBufMut {
    fn as_buf_ptr(&self) -> *const u8 {
        self.buf.as_buf_ptr()
    }

    fn buf_len(&self) -> usize {
        self.buf.buf_len()
    }

    fn buf_capacity(&self) -> usize {
        self.buf.buf_capacity()
    }
}

impl SetBufInit for PooledBufMut {
    unsafe fn set_buf_init(&mut self, len: usize)  {
        debug_assert!(len <= self.buf.len());
    }
}

unsafe impl IoBufMut for PooledBufMut {
    fn as_buf_mut_ptr(&mut self) -> *mut u8 {
        self.buf.as_mut_ptr()
    }

    fn as_mut_slice(&mut self) -> &mut [MaybeUninit<u8>] {
        unsafe {
            let slice =  self.buf.as_mut_slice();
            std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut MaybeUninit<u8>, slice.len())
        }
    }

    unsafe fn as_io_slice_mut(&mut self) -> IoSliceMut {
        let slice =  self.buf.as_mut_slice();
        unsafe { IoSliceMut::from_slice(slice) }
    }
}