// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc_::boxed::Box;

use crate::containers::Container;
use crate::iter::IntoIter;
use crate::ops::{Assign, Get, Len, Modify, Put, Set, WithOne};

impl<V> Container for Box<V> {
    type Key = usize;
    type Value = V;
}

impl<V> WithOne<V> for Box<V> {
    #[inline(always)]
    fn with_one(value: V) -> Self {
        Box::new(value)
    }
}

impl<V> Assign for Box<V> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<V> Get<usize> for Box<V> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        if *index == 0 { Some(&**self) } else { None }
    }
}

impl<V> Set<usize> for Box<V> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, index: usize, value: V) -> Option<V> {
        assert_eq!(index, 0);
        Some(core::mem::replace(&mut *self, value))
    }
}

impl<V> Modify<usize> for Box<V> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut V),
    {
        assert_eq!(*index, 0);
        f(&mut *self)
    }
}

impl<V> Put<V> for Box<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        Some(core::mem::replace(&mut *self, value))
    }
}

impl<V> Len for Box<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        1
    }
}

impl<V> IntoIter<usize> for Box<V> {
    type IntoIter = core::iter::Enumerate<core::iter::Once<V>>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        core::iter::once(*self).enumerate()
    }
}
