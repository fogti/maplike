// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std_::sync::{Arc, Weak};

use crate::containers::Container;
use crate::ops::{Assign, Clear, Get, IntoIter, Len, Modify, Put, Remove, Set, WithOne};

impl<V> Container for Arc<V> {
    type Key = usize;
    type Value = V;
}

impl<V> WithOne<V> for Arc<V> {
    #[inline(always)]
    fn with_one(value: V) -> Self {
        Arc::new(value)
    }
}

impl<V> Assign for Arc<V> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<V> Get<usize> for Arc<V> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        if *index == 0 { Some(&**self) } else { None }
    }
}

impl<V> Set<usize> for Arc<V> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, index: usize, value: V) -> Option<V> {
        assert_eq!(index, 0);
        Some(core::mem::replace(
            Arc::get_mut(self).expect("Arc is not uniquely owned"),
            value,
        ))
    }
}

impl<V> Modify<usize> for Arc<V> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut V),
    {
        assert_eq!(*index, 0);
        f(Arc::get_mut(self).expect("Arc is not uniquely owned"))
    }
}

impl<V> Put<V> for Arc<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        Some(core::mem::replace(
            Arc::get_mut(self).expect("Arc is not uniquely owned"),
            value,
        ))
    }
}

impl<V> Len for Arc<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        1
    }
}

impl<V> IntoIter<usize> for Arc<V> {
    type IntoIter = core::iter::Enumerate<core::iter::Once<V>>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        core::iter::once(match Arc::try_unwrap(self) {
            Ok(value) => value,
            Err(_) => panic!("Arc is not uniquely owned"),
        })
        .enumerate()
    }
}

impl<V> Container for Weak<V> {
    type Key = usize;
    type Value = V;
}

impl<V> Assign for Weak<V> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<V> Remove<usize> for Weak<V> {
    type Output = Option<V>;

    #[inline(always)]
    fn remove(&mut self, index: &usize) -> Option<V> {
        if *index == 0 {
            *self = Weak::new();
            None
        } else {
            None
        }
    }
}

impl<V> Clear for Weak<V> {
    #[inline(always)]
    fn clear(&mut self) {
        *self = Weak::new();
    }
}

impl<V> Len for Weak<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        usize::from(self.upgrade().is_some())
    }
}
