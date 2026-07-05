// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use tinyvec::{Array, ArrayVec};

use crate::{Assign, Clear, Container, Get, IntoIter, Len, Modify, Pop, Push, Put, Set, WithOne};

impl<A: Array> Container for ArrayVec<A> {
    type Key = usize;
    type Value = A::Item;
}

impl<A: Array> WithOne<A::Item> for ArrayVec<A> {
    #[inline(always)]
    fn with_one(element: A::Item) -> Self {
        let mut array_vec = ArrayVec::new();
        ArrayVec::push(&mut array_vec, element);

        array_vec
    }
}

impl<A: Array> Assign for ArrayVec<A> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<A: Array> Get<usize> for ArrayVec<A> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&A::Item> {
        self.as_slice().get(*index)
    }
}

impl<A: Array> Set<usize> for ArrayVec<A> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, index: usize, value: A::Item) {
        self[index] = value;
    }
}

impl<A: Array> Modify<usize> for ArrayVec<A> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut A::Item),
    {
        f(&mut self[*index]);
    }
}

impl<A: Array> Push<usize> for ArrayVec<A> {
    #[inline(always)]
    fn push(&mut self, value: A::Item) -> usize {
        ArrayVec::push(self, value);

        self.len() - 1
    }
}

impl<A: Array> Pop for ArrayVec<A> {
    #[inline(always)]
    fn pop(&mut self) -> Option<A::Item> {
        ArrayVec::pop(self)
    }
}

impl<A: Array> Put<A::Item> for ArrayVec<A> {
    #[inline(always)]
    fn put(&mut self, value: A::Item) -> Option<A::Item> {
        ArrayVec::push(self, value);

        None
    }
}

impl<A: Array> Clear for ArrayVec<A> {
    #[inline(always)]
    fn clear(&mut self) {
        ArrayVec::clear(self);
    }
}

impl<A: Array> Len for ArrayVec<A> {
    #[inline(always)]
    fn len(&self) -> usize {
        ArrayVec::len(self)
    }
}

impl<A: Array> IntoIter<usize> for ArrayVec<A> {
    type IntoIter = core::iter::Enumerate<<ArrayVec<A> as IntoIterator>::IntoIter>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}
