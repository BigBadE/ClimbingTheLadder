use std::alloc::{alloc, dealloc, Layout};
use std::{mem, ptr};
use std::any::TypeId;

pub struct AllocHandle {
    pointer: u64,
    type_id: TypeId,
    size: usize
}

impl AllocHandle {
    pub fn new<T>(allocing: &mut T) -> Self where T: Clone + 'static {
        unsafe {
            let size = mem::size_of_val(allocing);
            let pointer = alloc(Layout::from_size_align_unchecked(size, 0));
            ptr::write(pointer as *mut T, allocing.clone());

            return Self {
                pointer: pointer as u64,
                type_id: TypeId::of::<T>(),
                size
            };
        }
    }

    pub fn empty() -> Self {
        return Self {
            pointer: 0,
            type_id: TypeId::of::<AllocHandle>(),
            size: 0
        }
    }

    pub fn read<T>(&self) -> T where T: 'static {
        //Must save and read the same thing, but generics can't be kept in every situation, so the ID is checked
        assert_eq!(TypeId::of::<T>(), self.type_id);

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