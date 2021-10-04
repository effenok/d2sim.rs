use std::ops::Deref;
use std::ops::DerefMut;

pub struct InternalRef<T> {
    ptr: *mut T,
    initialized: bool,
}

impl<T> Deref for InternalRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        assert!(self.initialized, "reading uninitialized reference");

        unsafe {self.ptr.as_ref().unwrap()}
    }
}

impl<T> DerefMut for InternalRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(self.initialized, "reading uninitialized reference");

        unsafe { self.ptr.as_mut().unwrap() }
    }
}

impl<T> Default for InternalRef<T> {
    fn default() -> Self {
        InternalRef {
            ptr: std::ptr::null_mut(),
            initialized: false,
        }
    }
}

impl<T> InternalRef<T> {
    pub fn new() -> Self {
        InternalRef {
            ptr: std::ptr::null_mut(),
            initialized: false,
        }
    }

    pub fn set(&mut self, mut_ref: &mut T) {
        self.ptr = mut_ref as *mut T;
        self.initialized = true;
    }

    pub fn set_ptr(&mut self, ptr: *mut T) {
        self.ptr = ptr;
        self.initialized = true;
    }
}