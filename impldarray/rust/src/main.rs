use core::fmt;
use std::alloc::{AllocError, Allocator, Global, Layout};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::num::NonZeroUsize;
use std::ptr;
use std::ptr::NonNull;

#[derive(Debug)]

enum DynamicErr {
    OutOfBound,
}
struct DynamicArray<T: fmt::Debug, U: Allocator = Global> {
    capacity: NonZeroUsize,
    len: usize,
    data: NonNull<T>,
    alloc: U,
    _markertype: PhantomData<T>,
}

impl<T: fmt::Debug, U: Allocator> DynamicArray<T, U> {
    pub fn new(a: U) -> Self {
        let cap = 2;
        let layout = Layout::array::<T>(cap).unwrap();

        let ptr: NonNull<T> = a.allocate(layout).expect("Allocation Failed").cast();

        Self {
            capacity: NonZeroUsize::new(cap).unwrap(),
            len: 0,
            data: ptr,
            alloc: a,
            _markertype: PhantomData,
        }
    }
    #[inline]
    fn grow(&mut self) -> Result<NonNull<[u8]>, AllocError> {
        let old = self.capacity.get();
        self.capacity = self
            .capacity
            .checked_mul(NonZeroUsize::new(2).unwrap())
            .unwrap();

        unsafe {
            self.alloc.grow(
                self.data.cast(),
                Layout::array::<T>(old).unwrap(),
                Layout::array::<T>(self.capacity.get()).unwrap(),
            )
        }
    }
    fn shrink(&mut self) -> Result<NonNull<[u8]>, AllocError> {
        let old = self.capacity.get();
        self.capacity = NonZeroUsize::new(self.capacity.get() / 2).unwrap();

        unsafe {
            self.alloc.shrink(
                self.data.cast(),
                Layout::array::<T>(old).unwrap(),
                Layout::array::<T>(self.capacity.get()).unwrap(),
            )
        }
    }
    fn push(&mut self, value: T) {
        if self.capacity.get() < (self.len + 1) {
            match self.grow() {
                Ok(ptr) => self.data = ptr.cast(),
                Err(e) => print!("{}", e),
            }
        }
        unsafe {
            self.data.add(self.len).write(value);
        }
        self.len += 1;
    }
    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.data.as_ptr().add(self.len + 1))) }
        }
    }
    fn get_member(&mut self, member: usize) -> Result<T, DynamicErr> {
        if member <= self.len {
            unsafe { Ok(ptr::read(self.data.as_ptr().add(member))) }
        } else {
            Err((DynamicErr::OutOfBound))
        }
    }
}

macro_rules! dynamicArray{
    ( $( $x:expr ),* ) => {
        {
            let mut d = DynamicArray::new(Global);
            $(
                d.push($x);
            )*
            d
        }
    };
}

impl<T: fmt::Debug, U: Allocator> fmt::Display for DynamicArray<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut counter = 0;
        write!(f, "[").unwrap();
        while (counter + 1) < self.len {
            unsafe {
                write!(f, "{:?}, ", ptr::read(self.data.as_ptr().add(counter))).unwrap();
            }
            counter += 1;
        }
        if self.len != 0 {
            unsafe {
                write!(f, "{:?}]", ptr::read(self.data.as_ptr().add(counter))).unwrap();
            }
        } else {
            write!(f, "]").unwrap();
        }
        Ok(())
    }
}

fn main() {
    let mut d = dynamicArray! {1,2,3,4,5};
    d.pop();
    d.push(3);
    println!("{}", d);
    println!("{}", d.get_member(2).unwrap());
}
