// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2020 Andre Richter <andre.o.richter@gmail.com>

//! Common device driver code.

use core::{marker::PhantomData, ops};

pub struct  MMIODerefWrapper<T> {
    base_addr: usize,
    phantom: PhantomData<T>,
}

impl <T> MMIODerefWrapper<T> {
    pub const unsafe fn new(base_addr: usize) -> Self {
        Self {
            base_addr,
            phantom: PhantomData,
        }
    }

    fn ptr(&self) -> *const T {
        self.base_addr as *const _
    }
}

impl<T> ops::Deref for MMIODerefWrapper<T> {
    type  Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}