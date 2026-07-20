// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc_::collections::VecDeque;

use crate::containers::Container;
use crate::ops::{
    Assign, Clear, Get, Insert, IntoIter, Len, Modify, Pop, Push, Put, Remove, Set, WithOne,
};

impl<V> Container for VecDeque<V> {
    type Key = usize;
    type Value = V;
}

impl<V> WithOne<V> for VecDeque<V> {
    #[inline(always)]
    fn with_one(element: V) -> Self {
        let mut vecdeque = VecDeque::new();
        VecDeque::push_back(&mut vecdeque, element);

        vecdeque
    }
}

impl<V> Assign for VecDeque<V> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<V> Get<usize> for VecDeque<V> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        self.get(*index)
    }
}

impl<V> Set<usize> for VecDeque<V> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, index: usize, value: V) {
        self[index] = value;
    }
}

impl<V> Modify<usize> for VecDeque<V> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(&mut self[*index]);
    }
}

impl<V> Insert<usize> for VecDeque<V> {
    type Output = ();

    #[inline(always)]
    fn insert(&mut self, index: usize, value: V) -> () {
        VecDeque::insert(self, index, value);
    }
}

impl<V> Remove<usize> for VecDeque<V> {
    type Output = Option<V>;

    #[inline(always)]
    fn remove(&mut self, index: &usize) -> Option<V> {
        alloc_::collections::VecDeque::remove(self, *index)
    }
}

impl<V> Push<usize> for VecDeque<V> {
    #[inline(always)]
    fn push(&mut self, value: V) -> usize {
        VecDeque::push_back(self, value);

        self.len() - 1
    }
}

impl<V> Pop for VecDeque<V> {
    #[inline(always)]
    fn pop(&mut self) -> Option<V> {
        VecDeque::pop_back(self)
    }
}

impl<V> Put<V> for VecDeque<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        VecDeque::push_back(self, value);

        None
    }
}

impl<V> Clear for VecDeque<V> {
    #[inline(always)]
    fn clear(&mut self) {
        VecDeque::clear(self);
    }
}

impl<V> Len for VecDeque<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        VecDeque::len(self)
    }
}

impl<V> IntoIter<usize> for VecDeque<V> {
    type IntoIter = core::iter::Enumerate<alloc_::collections::vec_deque::IntoIter<V>>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}
