// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use _alloc::vec::Vec;

use crate::{Get, Insert, IntoIter, KeyedCollection, Pop, Push, Remove, Set};

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

impl<V: Clone> Insert<usize> for Vec<V> {
    #[inline(always)]
    fn insert(&mut self, index: usize, value: V) {
        // If the `Vec`'s len is too small, resize it and fill the elements
        // between with the same value as the new inserted one.
        //
        // This is somewhat of a hack, but it helps in making the `undoredo`
        // work.
        if index >= self.len() {
            self.resize(index + 1, value);
        }
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

impl<V> IntoIter<usize> for Vec<V> {
    type IntoIter = core::iter::Enumerate<_alloc::vec::IntoIter<V>>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}
