// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc_::vec::Vec;

use crate::{Assign, Clear, Container, Get, IntoIter, Len, Modify, Pop, Push, Put, Set, WithOne};

impl<V> Container for Vec<V> {
    type Key = usize;
    type Value = V;
}

impl<V> WithOne<V> for Vec<V> {
    #[inline(always)]
    fn with_one(element: V) -> Self {
        let mut vec = Vec::new();
        Vec::push(&mut vec, element);

        vec
    }
}

impl<V> Assign for Vec<V> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<V> Get<usize> for Vec<V> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        self.as_slice().get(*index)
    }
}

impl<V> Set<usize> for Vec<V> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, index: usize, value: V) {
        self[index] = value;
    }
}

impl<V> Modify<usize> for Vec<V> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(&mut self[*index]);
    }
}

impl<V> Push<usize> for Vec<V> {
    #[inline(always)]
    fn push(&mut self, value: V) -> usize {
        Vec::push(self, value);

        self.len() - 1
    }
}

impl<V> Pop for Vec<V> {
    #[inline(always)]
    fn pop(&mut self) -> Option<V> {
        Vec::pop(self)
    }
}

impl<V> Put<V> for Vec<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        Vec::push(self, value);

        None
    }
}

impl<V> Clear for Vec<V> {
    #[inline(always)]
    fn clear(&mut self) {
        Vec::clear(self);
    }
}

impl<V> Len for Vec<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        Vec::len(self)
    }
}

impl<V> IntoIter<usize> for Vec<V> {
    type IntoIter = core::iter::Enumerate<alloc_::vec::IntoIter<V>>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}
