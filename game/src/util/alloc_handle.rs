use std::alloc::{alloc, dealloc, Layout};
use std::{mem, ptr};

pub struct AllocHandle {
    pointer: u64,
    size: usize
}

impl AllocHandle {
    pub fn new<T>(allocing: &mut T) -> Self {
        unsafe {
            let size = mem::size_of_val(allocing);
            let pointer = alloc(Layout::from_size_align_unchecked(size, 0));
            ptr::copy_nonoverlapping(allocing as *mut T, pointer as *mut T, size);

            return Self {
                pointer: pointer as u64,
                size
            };
        }
    }

    pub fn empty() -> Self {
        return Self {
            pointer: 0,
            size: 0
        }
    }

    pub fn read<T>(&self) -> T {
        //Must save and read the same thing, but generics can't be kept in every situation, so we just trust the code.
        assert_eq!(mem::size_of::<T>(), self.size);

        unsafe {
            return ptr::read(self.pointer as *const u8 as *const T);
        }
    }
}

impl Drop for AllocHandle {
    fn drop(&mut self) {
        if self.size == 0 {
            return;
        }
        unsafe {
            dealloc(self.pointer as *mut u64 as *mut u8,
                    Layout::from_size_align_unchecked(self.size, 0));
        }
    }
}