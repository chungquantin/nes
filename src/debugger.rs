use std::{
    fmt::{Binary, Debug},
    marker::PhantomData,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct CpuDebugger<T: Binary + Debug> {
    _marker_data: PhantomData<T>,
}

impl<T> CpuDebugger<T>
where
    T: Binary + Debug,
{
    pub fn bin(self, value: T) {
        let x = value;
        println!("{x:#b}")
    }

    pub fn hex(self, value: T) {
        let x = value;
        println!("{:0x?}", x)
    }
}
