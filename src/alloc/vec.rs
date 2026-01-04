// SPDX-FileCopyrightText: 2025 maplike contributors
// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use _alloc::vec::Vec;

use crate::{Get, Insert, IntoIter, Keyed, Map, Pop, Push, Remove};

impl<V> Map for Vec<V> {
    type Item = V;
}

impl<V> Keyed for Vec<V> {
    type Key = usize;
}

impl<V> Get<usize> for Vec<V> {
    #[inline(always)]
    fn get(&self, key: &usize) -> Option<&V> {
        self.as_slice().get(*key)
    }
}

impl<V> Insert<usize> for Vec<V> {
    #[inline(always)]
    fn insert(&mut self, key: usize, value: V) {
        self[key] = value;
    }
}

impl<V> Remove<usize> for Vec<V> {
    #[inline(always)]
    fn remove(&mut self, key: &usize) -> Option<V> {
        Some(self.swap_remove(*key))
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
