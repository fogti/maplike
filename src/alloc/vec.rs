// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc_::vec::Vec;

use crate::{Clear, Get, IntoIter, KeyedCollection, Len, Pop, Push, Remove, Set};

impl<V> KeyedCollection for Vec<V> {
    type Key = usize;
    type Value = V;
}

impl<V> Get<usize> for Vec<V> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        self.as_slice().get(*index)
    }
}

impl<V> Set<usize> for Vec<V> {
    #[inline(always)]
    fn set(&mut self, index: usize, value: V) {
        self[index] = value;
    }
}

impl<V> Remove<usize> for Vec<V> {
    #[inline(always)]
    fn remove(&mut self, index: &usize) -> Option<V> {
        Some(self.swap_remove(*index))
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
