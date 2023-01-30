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
            return &ptr::read(self.pointer as *const u8 as *const T);
        }
    }

    pub fn deref<T>(self) -> T where T: 'static {
        //Must save and read the same thing, but generics can't be kept in every situation, so the ID is checked
        assert_eq!(TypeId::of::<T>(), self.type_id);

        unsafe {
            return ptr::read(self.pointer as *const u8 as *const T);
        }
    }

    pub fn deref_boxed<T>(self) -> Box<T> where T: 'static {
        //Must save and read the same thing, but generics can't be kept in every situation, so the ID is checked
        assert_eq!(TypeId::of::<T>(), self.type_id);

        unsafe {
            return Box::from_raw(self.pointer as *mut u8 as *mut T);
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