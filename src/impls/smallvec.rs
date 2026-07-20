// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use smallvec::{Array, SmallVec};

use crate::containers::Container;
use crate::iter::IntoIter;
use crate::ops::{Assign, Clear, Get, Len, Modify, Pop, Push, Put, Resize, Set, WithOne};

impl<A: Array> Container for SmallVec<A> {
    type Key = usize;
    type Value = A::Item;
}

impl<A: Array> WithOne<A::Item> for SmallVec<A> {
    #[inline(always)]
    fn with_one(element: A::Item) -> Self {
        let mut small_vec = SmallVec::new();
        SmallVec::push(&mut small_vec, element);

        small_vec
    }
}

impl<A: Array> Assign for SmallVec<A> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<A: Array> Get<usize> for SmallVec<A> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&A::Item> {
        self.as_slice().get(*index)
    }
}

impl<A: Array> Set<usize> for SmallVec<A> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, index: usize, value: A::Item) {
        self[index] = value;
    }
}

impl<A: Array> Modify<usize> for SmallVec<A> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut A::Item),
    {
        f(&mut self[*index]);
    }
}

impl<A: Array> Push<usize> for SmallVec<A> {
    #[inline(always)]
    fn push(&mut self, value: A::Item) -> usize {
        SmallVec::push(self, value);

        self.len() - 1
    }
}

impl<A: Array> Pop for SmallVec<A> {
    #[inline(always)]
    fn pop(&mut self) -> Option<A::Item> {
        SmallVec::pop(self)
    }
}

impl<A: Array> Put<A::Item> for SmallVec<A> {
    #[inline(always)]
    fn put(&mut self, value: A::Item) -> Option<A::Item> {
        SmallVec::push(self, value);

        None
    }
}

impl<A: Array> Clear for SmallVec<A> {
    #[inline(always)]
    fn clear(&mut self) {
        SmallVec::clear(self);
    }
}

impl<A: Array> Len for SmallVec<A> {
    #[inline(always)]
    fn len(&self) -> usize {
        SmallVec::len(self)
    }
}

impl<A: Array> Resize for SmallVec<A> {
    #[inline(always)]
    fn resize(&mut self, new_len: usize, value: A::Item)
    where
        A::Item: Clone,
    {
        SmallVec::resize(self, new_len, value);
    }
}

impl<A: Array> IntoIter<usize> for SmallVec<A> {
    type IntoIter = core::iter::Enumerate<<SmallVec<A> as IntoIterator>::IntoIter>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}
