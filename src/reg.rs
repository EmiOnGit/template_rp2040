use core::{cell::UnsafeCell, ptr};

/// Read-Only register
#[repr(transparent)]
pub struct RReg<T>
where
    T: Copy,
{
    register: *const T,
}
impl<T> RReg<T>
where
    T: Copy,
{
    /// Reads the value of the register
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(self.register) }
    }
}
/// Read-Write register
#[repr(transparent)]
pub struct RwReg<T>
where
    T: Copy,
{
    register: *mut T,
}

impl<T> RwReg<T>
where
    T: Copy,
{
    /// Reads the value of the register
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(self.register) }
    }
    #[inline(always)]
    pub fn write(&self, v: T) {
        unsafe { ptr::write_volatile(self.register, v) }
    }
    pub fn modify<F>(&self, f: F)
    where
        F: FnOnce(T) -> T,
    {
        // TODO do we need to make both volatile operations?
        self.write(f(self.read()));
    }
}
