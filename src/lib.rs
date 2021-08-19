#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]

use std::{ffi::c_void, mem::ManuallyDrop};
use thin_trait_object::thin_trait_object;

pub fn resolve_double(ptr: *mut c_void) -> u32 {
    let ptr = ManuallyDrop::new(unsafe { Box::from_raw(ptr.cast::<Box<dyn Empty>>()) });
    ptr.add()
}

pub fn resolve_typed<T: Empty>(ptr: *mut c_void) -> u32 {
    let ptr = unsafe { &*ptr.cast::<T>() };
    ptr.add()
}

pub fn resolve_thin(ptr: *mut c_void) -> u32 {
    let ptr = ManuallyDrop::new(unsafe { BoxedEmpty::from_raw(ptr.cast()) });
    ptr.add()
}

pub fn callback_double(ptr: *mut c_void, value: u32) -> u32 {
    let mut ptr = ManuallyDrop::new(unsafe { Box::from_raw(ptr.cast::<Box<dyn Callback>>()) });
    ptr.call(value)
}

pub fn callback_typed<T: Callback>(ptr: *mut c_void, value: u32) -> u32 {
    let ptr = unsafe { &mut *ptr.cast::<T>() };
    ptr.call(value)
}

pub fn callback_thin(ptr: *mut c_void, value: u32) -> u32 {
    let mut ptr = ManuallyDrop::new(unsafe { BoxedCallback::from_raw(ptr.cast()) });
    ptr.call(value)
}

#[thin_trait_object]
pub trait Empty {
    fn add(&self) -> u32;
}

impl Empty for u32 {
    fn add(&self) -> u32 {
        *self + *self
    }
}

#[thin_trait_object]
pub trait Callback {
    fn call(&mut self, value: u32) -> u32;
}

impl<F: FnMut(u32) -> u32> Callback for F {
    #[inline]
    fn call(&mut self, value: u32) -> u32 {
        (self)(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_double() {
        let mut ptr = Box::new(Box::new(2_u32) as Box<dyn Empty>);
        let ptr = (&mut *ptr) as *mut Box<dyn Empty>;
        assert_eq!(4, resolve_double(ptr.cast()));
        assert_eq!(4, resolve_double(ptr.cast()));
        // unsafe { Box::from_raw(ptr) };
    }

    #[test]
    fn test_resolve_typed() {
        let mut ptr = Box::new(2_u32);
        let ptr = (&mut *ptr) as *mut u32;
        assert_eq!(4, resolve_typed::<u32>(ptr.cast()));
        assert_eq!(4, resolve_typed::<u32>(ptr.cast()));
        // unsafe { Box::from_raw(ptr) };
    }

    #[test]
    fn test_resolve_thin() {
        let ptr = BoxedEmpty::new(2_u32);
        assert_eq!(4, resolve_thin(ptr.as_raw().cast()));
        assert_eq!(4, resolve_thin(ptr.as_raw().cast()));
    }

    #[test]
    fn test_callback_double() {
        let mut ptr = Box::new(Box::new(|value: u32| value + value) as Box<dyn Callback>);
        let ptr = (&mut *ptr) as *mut Box<dyn Callback>;
        assert_eq!(4, callback_double(ptr.cast(), 2));
    }

    #[test]
    fn test_callback_thin() {
        let ptr = BoxedCallback::new(|value: u32| value + value);
        assert_eq!(4, callback_thin(ptr.as_raw().cast(), 2));
        assert_eq!(4, callback_thin(ptr.as_raw().cast(), 2));
    }
}
