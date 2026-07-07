// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{Assign, Clear, Container, Get, IntoIter, Len, Modify, Put, Remove, Set, WithOne};

impl<V> Container for Option<V> {
    type Key = usize;
    type Value = V;
}

impl<V> WithOne<V> for Option<V> {
    #[inline(always)]
    fn with_one(element: V) -> Self {
        Some(element)
    }
}

impl<V> Assign for Option<V> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<V> Get<usize> for Option<V> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        if *index == 0 { self.as_ref() } else { None }
    }
}

impl<V> Set<usize> for Option<V> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, index: usize, value: V) -> Option<V> {
        assert_eq!(index, 0);
        self.replace(value)
    }
}

impl<V> Modify<usize> for Option<V> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut V),
    {
        assert_eq!(*index, 0);
        f(self.as_mut().expect("no value under key"));
    }
}

impl<V> Remove<usize> for Option<V> {
    #[inline(always)]
    fn remove(&mut self, index: &usize) -> Option<V> {
        if *index == 0 { self.take() } else { None }
    }
}

impl<V> Put<V> for Option<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        self.replace(value)
    }
}

impl<V> Clear for Option<V> {
    #[inline(always)]
    fn clear(&mut self) {
        *self = None;
    }
}

impl<V> Len for Option<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.is_some().into()
    }
}

impl<V> IntoIter<usize> for Option<V> {
    type IntoIter = core::iter::Enumerate<core::option::IntoIter<V>>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}
