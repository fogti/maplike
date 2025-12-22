// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use _alloc::vec::Vec;

use crate::{Get, IntoIter, Keyed, Map, Pop, Push};

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
