use std::alloc::{dealloc, Layout};
use std::{mem, ptr};
use std::any::TypeId;

pub struct AllocHandle {
    pointer: u64,
    type_id: TypeId,
    size: usize,
}

impl AllocHandle {
    pub fn new<T>(allocing: T) -> Self where T: 'static {
        let size = mem::size_of_val(&allocing);
        let reference = Box::leak(Box::new(allocing));

        return Self {
            pointer: reference as *mut T as u64,
            type_id: TypeId::of::<T>(),
            size,
        };
    }

    pub fn empty() -> Self {
        return Self {
            pointer: 0,
            type_id: TypeId::of::<AllocHandle>(),
            size: 0,
        };
    }

    pub fn read<T>(&self) -> &T where T: 'static {
        //Must save and read the same thing, but generics can't be kept in every situation, so the ID is checked
        assert_eq!(TypeId::of::<T>(), self.type_id);

        unsafe {
            //Even though this is leaked, the reference lifetime is locked to the alloc lifetime which will drop it.
            return Box::leak(Box::new(ptr::read(self.pointer as *const u8 as *const T)));
        }
    }

    pub fn deref<T>(self) -> T where T: 'static {
        //Must save and read the same thing, but generics can't be kept in every situation, so the ID is checked
        assert_eq!(TypeId::of::<T>(), self.type_id);

        let reference = Box::leak(Box::new(self));

        unsafe {
            return ptr::read(reference.pointer as *const u8 as *const T);
        }
    }

    pub(crate) fn convert<T: 'static>(handles: Vec<AllocHandle>) -> Vec<T> {
        let mut output = Vec::new();
        for handle in handles {
            output.push(handle.deref());
        }
        return output;
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