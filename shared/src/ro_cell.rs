use std::{
    cell::UnsafeCell,
    fmt::{self, Display},
};

// Read-only cell. It's safe to use this in a static variable, but it's not safe
// to mutate it. This is useful for storing static data that is expensive to
// initialize, but is immutable once.
pub struct RoCell<T>(UnsafeCell<Option<T>>);

unsafe impl<T> Sync for RoCell<T> {}

impl<T> RoCell<T> {
    #[inline]
    pub const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }

    #[inline]
    pub fn init(&self, value: T) {
        unsafe {
            *self.0.get() = Some(value);
        }
    }

    #[inline]
    pub fn with<F>(&self, f: F)
    where
        F: FnOnce() -> T,
    {
        self.init(f());
    }
}

impl<T> AsRef<T> for RoCell<T> {
    fn as_ref(&self) -> &T {
        unsafe { (*self.0.get()).as_ref().unwrap() }
    }
}

impl<T> Display for RoCell<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}
